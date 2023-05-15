use std::sync::Arc;

use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;

mod sqlx_queue;
mod sqlx_shorturl;

#[tokio::main]
async fn main() {
    setup_log();
    let config = Arc::new(load_config());
    let pool = Arc::new(setup_pg_pool(&config.database_url, config.database_max_connections).await);

    if let Some(conf) = config.queue_test_config.clone() {
        let pool = pool.clone();
        tokio::spawn(async move {
            sqlx_queue::app::run(pool, &conf).await
        });
    }

    for conf in config.shorturl_test_config.clone() {
        let pool = pool.clone();
        tokio::spawn(async move {
            sqlx_shorturl::app::run(pool, &conf).await
        });
    }

    // wait
    sleep(config.test_duration_ms).await;
}

fn sleep(sleep_duration: u64) -> tokio::time::Sleep {
    tokio::time::sleep(tokio::time::Duration::from_millis(sleep_duration))
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

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
struct Config {
    database_url: String,
    database_max_connections: u32,
    test_duration_ms: u64,
    queue_test_config: Option<sqlx_queue::app::Config>,
    shorturl_test_config: Vec<sqlx_shorturl::app::Config>,
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
