mod db {
    use serde::de::DeserializeOwned;
    use serde::Serialize;
    use sqlx::{Executor, Postgres, Row};

    pub struct Task<T> {
        pub id: i64,
        pub data: T,
    }

    pub async fn queue_system_destroy<E>(executor: &E) -> Result<(), String>
        where for<'a> &'a E: Executor<'a, Database=Postgres> {
        sqlx::query(r#"
            drop table if exists tasks
        "#)
            .execute(executor)
            .await
            .map(|_| ())
            .map_err(|e| format!("Unable to destroy tasks table: {:?}", e))
    }

    pub async fn queue_system_init<E>(executor: &E) -> Result<(), String>
        where for<'a> &'a E: Executor<'a, Database=Postgres> {
        sqlx::query(r#"
            create table if not exists tasks (
                id bigint not null primary key,
                topic varchar(255) not null,
                created_at timestamptz not null,
                available_after timestamptz not null,
                num_attempts smallint not null,
                data jsonb
            );
        "#)
            .execute(executor)
            .await
            .map(|_| ())
            .map_err(|e| format!("Unable to init tasks table: {:?}", e))?;
        sqlx::query(r#"
            create index if not exists tasks_poll on tasks (topic, created_at, num_attempts, available_after);
        "#)
            .execute(executor)
            .await
            .map(|_| ())
            .map_err(|e| format!("Unable to init tasks table: {:?}", e))
    }

    pub struct QueuePushParam<'a, T> {
        pub id: i64,
        pub topic: &'a str,
        pub data: &'a T,
    }

    pub async fn queue_push<'c, 'a, E: Executor<'c, Database=Postgres>, T: Serialize + Send + Sync>(executor: E, param: &QueuePushParam<'a, T>) -> Result<(), String> {
        let result = sqlx::query(
            "insert into tasks(id, topic, num_attempts, data, available_after, created_at) values ($1, $2, 0, $3, now(), now())")
            .bind(param.id)
            .bind(param.topic)
            .bind(sqlx::types::Json(param.data))
            .execute(executor)
            .await
            .map_err(|e| format!("Unable to insert message to DB: {:?}", e))
            .map(|_| ());
        result
    }

    pub struct QueuePollParam<'a> {
        pub topic: &'a str,
        pub max_num_attempts: i16,
        pub lease_duration_ms: u32,
    }

    pub async fn queue_poll<'c, E, T>(executor: &'c E, param: &QueuePollParam<'c>) -> Result<Option<Task<T>>, String>
        where &'c E: Executor<'c, Database=Postgres>,
              T: DeserializeOwned + Unpin + Send {
        sqlx::query(
            r#"
        update tasks set
            available_after = now() + ($1 * interval '1 ms'),
            num_attempts = num_attempts + 1
        where id in (
            select id
            from tasks
            where topic = $2
            and available_after <= now()
            and num_attempts < $3
            and (true or created_at > now() - ($4 * interval '1 ms'))
            limit 1
            for update skip locked
        )
        returning id, data
        "#)
            .bind(param.lease_duration_ms as i64)
            .bind(param.topic)
            .bind(param.max_num_attempts)
            .bind(86400000) // TODO: move this to parameter
            .map(|row| {
                let sqlx::types::Json(data) = row.get::<sqlx::types::Json<T>, &str>("data");
                Task {
                    id: row.get("id"),
                    data,
                }
            })
            .fetch_optional(executor)
            .await
            .map_err(|e| format!("Unable to fetch message from DB: {:?}", e))
    }

    pub async fn queue_delete<'c, E: Executor<'c, Database=Postgres>>(executor: E, id: i64) -> Result<(), String> {
        let result = sqlx::query("delete from tasks where id = $1")
            .bind(id)
            .execute(executor)
            .await
            .map_err(|e| format!("Unable to delete message from DB: {:?}", e))
            .map(|_| ());
        result
    }
}

pub mod app {
    use std::sync::Arc;
    use sqlx::{Executor, Pool, Postgres};
    use super::db as db;

    #[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
    pub struct Config {
        num_topic: u8,
        message_size_bytes: u64,
        consumer_sleep_time_ms: u64,
        num_consumer_per_topic: u8,
        num_producer_per_topic: u8,
    }

    pub async fn run(executor: Arc<Pool<Postgres>>, config: &Config) {
        db::queue_system_destroy(&executor).await.expect("Unable to destroy existing system");
        db::queue_system_init(&executor).await.expect("Unable to init sqlx_scheduler");

        // spawn consumers
        for i in 0..config.num_topic {
            for j in 0..config.num_consumer_per_topic {
                let pool = executor.clone();
                let sleep_ms = config.consumer_sleep_time_ms;
                tokio::spawn(async move {
                    run_consumer(&pool, &create_topic(i), j, sleep_ms).await;
                });
            }
        }

        // spawn producers
        let message = Arc::new("1".repeat(config.message_size_bytes as usize));
        for i in 0..config.num_topic {
            for j in 0..config.num_producer_per_topic {
                let pool = executor.clone();
                let message = message.clone();
                let publisher_id = (i * config.num_producer_per_topic + j) as i32;
                tokio::spawn(async move {
                    run_publisher(&pool, &create_topic(i), publisher_id, &message).await;
                });
            }
        }
    }

    fn create_topic(i: u8) -> String {
        format!("topic-{}", i)
    }

    fn sleep(sleep_duration: u64) -> tokio::time::Sleep {
        tokio::time::sleep(tokio::time::Duration::from_millis(sleep_duration))
    }

    fn get_epoch_ms() -> u128 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis()
    }

    async fn run_publisher<E, T>(executor: &E, topic: &str, publisher_id: i32, message: &T)
        where for<'a> &'a E: Executor<'a, Database=Postgres>,
              T: serde::Serialize + Send + Sync {
        let mut id_generator = snowflake::SnowflakeIdGenerator::new(publisher_id / 32, publisher_id % 32);
        loop {
            let push_param = db::QueuePushParam {
                id: id_generator.generate(),
                topic: &topic,
                data: message,
            };

            let start = get_epoch_ms();
            let pushed = db::queue_push(executor, &push_param).await;
            log::info!("{}\t{}\t{}\t{}", publisher_id, get_epoch_ms(), "TASK_PUSH_LATENCY", get_epoch_ms() - start);
            log::info!("{}\t{}\t{}\t{}", publisher_id, get_epoch_ms(), "TASK_PUSHED", push_param.id);

            if let Err(e) = pushed {
                log::warn!("Failed to publish msg to queue: {}", e);
            }
        }
    }

    async fn run_consumer<E>(executor: &E, topic: &str, consumer_id: u8, sleep_ms: u64)
        where for<'a> &'a E: Executor<'a, Database=Postgres> {
        let poll_param = db::QueuePollParam {
            topic,
            max_num_attempts: 1000,
            lease_duration_ms: 30000,
        };
        let consumer_id = format!("{}-{}", topic, consumer_id);

        loop {
            let start = get_epoch_ms();
            let polled: Result<Option<db::Task<String>>, String> = db::queue_poll(executor, &poll_param).await;
            log::info!("{}\t{}\t{}\t{}", consumer_id, get_epoch_ms(), "TASK_POLL_LATENCY", get_epoch_ms() - start);

            match polled {
                Err(e) => log::warn!("Failed to poll msg from the queue topic={}: {}", topic, e),
                Ok(None) => {
                    log::info!("{}\t{}\t{}\t{}", consumer_id, get_epoch_ms(), "SLEEP", sleep_ms);
                    sleep(sleep_ms).await
                }
                Ok(Some(task)) => {
                    log::info!("{}\t{}\t{}\t{}", consumer_id, get_epoch_ms(), "TASK_POLLED", task.id);

                    let start = get_epoch_ms();
                    let deleted = db::queue_delete(executor, task.id).await;
                    log::info!("{}\t{}\t{}\t{}", consumer_id, get_epoch_ms(), "TASK_DELETE_LATENCY", get_epoch_ms() - start);
                    log::info!("{}\t{}\t{}\t{}", consumer_id, get_epoch_ms(), "TASK_DELETED", task.id);

                    if let Err(e) = deleted {
                        log::warn!("Failed to delete msg ID={}: {}", task.id, e);
                    }
                }
            }
        }
    }
}