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