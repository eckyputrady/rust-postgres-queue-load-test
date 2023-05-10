use std::sync::Arc;

use serde::{Deserialize, Serialize};
use sqlx::{Executor, Pool, Postgres};
use sqlx::postgres::PgPoolOptions;
use tokio::time::Sleep;

use crate::sqlx_queue::{queue_delete, queue_poll, queue_push, QueuePollParam, QueuePushParam, Task};

mod sqlx_queue;

#[tokio::main]
async fn main() {
    setup_log();
    let config = Arc::new(load_config());
    let pool = Arc::new(setup_pg_pool(&config.database_url, config.database_max_connections).await);
    sqlx_queue::queue_system_destroy(&pool).await.expect("Unable to destroy existing system");
    sqlx_queue::queue_system_init(&pool).await.expect("Unable to init sqlx_scheduler");

    // spawn consumers
    for i in 0..config.num_topic {
        for j in 0..config.num_consumer_per_topic {
            let pool = pool.clone();
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
            let pool = pool.clone();
            let message = message.clone();
            let publisher_id = (i * config.num_producer_per_topic + j) as i32;
            tokio::spawn(async move {
                run_publisher(&pool, &create_topic(i), publisher_id, &message).await;
            });
        }
    }

    // wait
    sleep(config.test_duration_ms).await;
}

fn create_topic(i: u8) -> String {
    format!("topic-{}", i)
}

fn sleep(sleep_duration: u64) -> Sleep {
    tokio::time::sleep(tokio::time::Duration::from_millis(sleep_duration))
}

fn get_epoch_ms() -> u128 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

async fn setup_pg_pool(db_url: &str, max_connections: u32) -> Pool<Postgres> {
    PgPoolOptions::new()
        .max_connections(max_connections)
        .connect(db_url)
        .await
        .expect("Unable to connect to postgres")
}

fn setup_log() {
    fern::Dispatch::new()
        // Perform allocation-free log formatting
        .format(|out, message, _record| {
            out.finish(format_args!(
                "{}",
                message
            ))
        })
        // Add blanket level filter -
        .level(log::LevelFilter::Info)
        .level_for("sqlx", log::LevelFilter::Error)
        .chain(std::io::stdout())
        // Apply globally
        .apply()
        .expect("Unable to setup logging");
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    database_url: String,
    database_max_connections: u32,
    num_topic: u8,
    message_size_bytes: u64,
    test_duration_ms: u64,
    consumer_sleep_time_ms: u64,
    num_consumer_per_topic: u8,
    num_producer_per_topic: u8,
}

fn load_config() -> Config {
    use config::{Config, File};
    Config::builder()
        .add_source(File::with_name("config"))
        .build()
        .expect("Unable to build config")
        .try_deserialize()
        .expect("Unable to parse config")
}

async fn run_publisher<E, T>(executor: &E, topic: &str, publisher_id: i32, message: &T)
    where for<'a> &'a E: Executor<'a, Database=Postgres>,
          T: Serialize + Send + Sync {
    let mut id_generator = snowflake::SnowflakeIdGenerator::new(publisher_id / 32, publisher_id % 32);
    loop {
        let push_param = QueuePushParam {
            id: id_generator.generate(),
            topic: &topic,
            data: message,
        };

        let start = get_epoch_ms();
        let pushed = queue_push(executor, &push_param).await;
        log::info!("{}\t{}\t{}\t{}", publisher_id, get_epoch_ms(), "TASK_PUSH_LATENCY", get_epoch_ms() - start);
        log::info!("{}\t{}\t{}\t{}", publisher_id, get_epoch_ms(), "TASK_PUSHED", push_param.id);

        if let Err(e) = pushed {
            log::warn!("Failed to publish msg to queue: {}", e);
        }
    }
}

async fn run_consumer<E>(executor: &E, topic: &str, consumer_id: u8, sleep_ms: u64)
    where for<'a> &'a E: Executor<'a, Database=Postgres> {
    let poll_param = QueuePollParam {
        topic,
        max_num_attempts: 1000,
        lease_duration_ms: 30000,
    };
    let consumer_id = format!("{}-{}", topic, consumer_id);

    loop {
        let start = get_epoch_ms();
        let polled: Result<Option<Task<String>>, String> = queue_poll(executor, &poll_param).await;
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
                let deleted = queue_delete(executor, task.id).await;
                log::info!("{}\t{}\t{}\t{}", consumer_id, get_epoch_ms(), "TASK_DELETE_LATENCY", get_epoch_ms() - start);
                log::info!("{}\t{}\t{}\t{}", consumer_id, get_epoch_ms(), "TASK_DELETED", task.id);

                if let Err(e) = deleted {
                    log::warn!("Failed to delete msg ID={}: {}", task.id, e);
                }
            }
        }
    }
}
