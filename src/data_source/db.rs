use sqlx::{SqlitePool, query};

pub async fn test(pool: &SqlitePool) -> Vec<Option<i64>> {
    query!("insert into users (id) values (1)")
        .execute(pool)
        .await
        .unwrap();

    query!("select id from users")
        .fetch_all(pool)
        .await
        .unwrap()
        .iter()
        .map(|r| r.id)
        .collect()
}
