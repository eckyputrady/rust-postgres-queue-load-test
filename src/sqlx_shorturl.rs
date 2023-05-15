pub mod db {
    use sqlx::{Executor, Postgres};

    #[derive(sqlx::FromRow)]
    pub struct ShortUrl {
        pub short_url: String,
        pub long_url: String,
        pub created_at: i64,
    }

    pub async fn system_destroy<E>(executor: &E, table_name: &str) -> Result<(), String>
        where for<'a> &'a E: Executor<'a, Database=Postgres> {
        sqlx::query(&format!(r#"drop table if exists {}"#, table_name))
            .execute(executor)
            .await
            .map(|_| ())
            .map_err(|e| format!("Unable to destroy short_url table: {:?}", e))
    }

    pub async fn system_init<E>(executor: &E, table_name: &str, num_partitions: u8) -> Result<(), String>
        where for<'a> &'a E: Executor<'a, Database=Postgres> {
        let p = if num_partitions > 0 {
            "partition by hash (short_url)"
        } else {
            ""
        };
        sqlx::query(&format!(r#"
            create table if not exists {} (
                short_url varchar(255) not null primary key,
                created_at bigint not null,
                long_url varchar(2048) not null
            )
            {};
        "#, table_name, p))
            .execute(executor)
            .await
            .map(|_| ())
            .map_err(|e| format!("Unable to init {} table: {:?}", table_name, e))?;

        for i in 0..num_partitions {
            let partition_table_name = format!("{}_{}", table_name, i);
            sqlx::query(&format!(r#"
                    create table if not exists {} partition of {} for values with (modulus {}, remainder {});
                "#, partition_table_name, table_name, num_partitions, i))
                .execute(executor)
                .await
                .map(|_| ())
                .map_err(|e| format!("Unable to init {} table: {:?}", partition_table_name, e))?;
        };
        Ok(())
    }

    pub async fn add<E>(executor: &E, table_name: &str, short_url: &ShortUrl) -> Result<(), String>
        where for<'a> &'a E: Executor<'a, Database=Postgres> {
        sqlx::query(
            &format!(r#"
                insert into {} (created_at, short_url, long_url)
                values ($1, $2, $3)
                "#, table_name)
            )
            .bind(short_url.created_at)
            .bind(&short_url.short_url)
            .bind(&short_url.long_url)
            .execute(executor)
            .await
            .map(|_| ())
            .map_err(|e| format!("Unable to add into short_url table: {:?}", e))
    }

    pub async fn get<E>(executor: &E, table_name: &str, short_url: &str) -> Result<Option<ShortUrl>, String>
        where for<'a> &'a E: Executor<'a, Database=Postgres> {
        sqlx::query_as(
            &format!(r#"
                select created_at, short_url, long_url
                from {}
                where short_url = $1
            "#, table_name)
        )
            .bind(short_url)
            .fetch_optional(executor)
            .await
            .map_err(|e| format!("Unable to add into short_url table: {:?}", e))
    }
}

pub mod app {
    use std::sync::Arc;

    use sqlx::{Executor, Pool, Postgres};

    use crate::sqlx_shorturl::db::ShortUrl;

    use super::db as db;

    #[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
    pub struct Config {
        num_table_partitions: u8,
        num_writers: u16,
        num_readers: u16,
    }

    pub async fn run(executor: Arc<Pool<Postgres>>, config: &Config) {
        let table_name = if config.num_table_partitions > 0 {
            "short_url_partitioned"
        } else {
            "short_url"
        };
        // db::system_destroy(&executor, table_name).await.expect("Unable to destroy short_url");
        db::system_init(&executor, table_name, config.num_table_partitions).await.expect("Unable to init short_url");

        for _ in 0..config.num_readers {
            let pool = executor.clone();
            tokio::spawn(async move {
                run_reader(&pool, table_name).await;
            });
        }

        for _ in 0..config.num_writers {
            let pool = executor.clone();
            tokio::spawn(async move {
                run_writer(&pool, table_name).await;
            });
        }
    }

    async fn run_writer<E>(executor: &E, table_name: &str)
        where for<'a> &'a E: Executor<'a, Database=Postgres> {
        loop {
            let random = rand::random::<u32>();
            let short_url = ShortUrl {
                short_url: random.to_string(),
                long_url: format!("looooooong/{}", random).to_string(),
                created_at: get_epoch_ms() as i64,
            };
            let start = get_epoch_ms();
            let result = db::add(executor, table_name, &short_url).await;
            let end = get_epoch_ms();
            if let Err(e) = result {
                log::info!("{}\t{}\t{}\t{}\t{}", end, table_name, "ADD_SHORT_URL_FAIL", end - start, e);
            } else {
                log::info!("{}\t{}\t{}\t{}\t", end, table_name, "ADD_SHORT_URL_SUCC", end - start);
            }
        }
    }

    async fn run_reader<E>(executor: &E, table_name: &str)
        where for<'a> &'a E: Executor<'a, Database=Postgres> {
        loop {
            let short_url = rand::random::<u32>().to_string();
            let start = get_epoch_ms();
            let result = db::get(executor, table_name, &short_url).await;
            let end = get_epoch_ms();
            match result {
                Err(e) =>
                    log::info!("{}\t{}\t{}\t{}\t{}", end, table_name, "GET_SHORT_URL_FAIL", end - start, e),
                Ok(None) =>
                    log::info!("{}\t{}\t{}\t{}\t", end, table_name, "GET_SHORT_URL_NOT_FOUND", end - start),
                Ok(Some(_)) =>
                    log::info!("{}\t{}\t{}\t{}\t", end, table_name, "GET_SHORT_URL_FOUND", end - start)
            };
        }
    }

    fn get_epoch_ms() -> u128 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis()
    }
}