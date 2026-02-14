use crate::data_source::{MediaState, MediaType};
use serde::Deserialize;
use sqlx::{SqlitePool, query, query_as};
use thiserror::Error;
use time::OffsetDateTime;

#[derive(Debug, Error)]
pub enum DbError {
    #[error("Database query failed: {0}")]
    QueryError(#[from] sqlx::Error),
    #[error("User with this Google account ID already exists: {0}")]
    UserWithGoogleAccountIdAlreadyExists(String),
}

#[derive(Debug, Deserialize)]
pub struct User {
    pub id: i64,
    #[allow(unused)]
    pub google_account_id: Option<String>,
    pub email: Option<String>,
    pub username: Option<String>,
    pub picture_url: Option<String>,
    pub created_at: OffsetDateTime,
}

pub async fn create_user(
    pool: &SqlitePool,
    google_account_id: Option<&str>,
    email: Option<&str>,
    username: Option<&str>,
    picture_url: Option<&str>,
) -> Result<User, DbError> {
    query_as!(
        User,
        r#"
INSERT INTO users (google_account_id, email, username, picture_url)
VALUES (?, ?, ?, ?)
RETURNING *"#,
        google_account_id,
        email,
        username,
        picture_url,
    )
    .fetch_one(pool)
    .await
    .map_err(|e| {
        if let sqlx::Error::Database(db_err) = &e
            && db_err.message() == "UNIQUE constraint failed: users.google_account_id"
        {
            DbError::UserWithGoogleAccountIdAlreadyExists(
                google_account_id.unwrap_or_default().to_string(),
            )
        } else {
            DbError::QueryError(e)
        }
    })
}

pub async fn get_user_by_google_account_id(
    pool: &SqlitePool,
    google_account_id: &str,
) -> Result<Option<User>, DbError> {
    query_as!(
        User,
        r#"
SELECT *
FROM users
WHERE google_account_id = ?"#,
        google_account_id,
    )
    .fetch_optional(pool)
    .await
    .map_err(DbError::QueryError)
}

pub async fn get_user_by_id(pool: &SqlitePool, id: i64) -> Result<Option<User>, DbError> {
    query_as!(
        User,
        r#"
SELECT *
FROM users
WHERE id = ?"#,
        id,
    )
    .fetch_optional(pool)
    .await
    .map_err(DbError::QueryError)
}

// TODO: Delete all data from this user from all data tables
pub async fn delete_user(pool: &SqlitePool, id: i64) -> Result<(), DbError> {
    match query!(
        r#"
DELETE FROM media
WHERE user_id = ?"#,
        id,
    )
    .execute(pool)
    .await
    {
        Ok(_) => Ok(()),
        Err(e) => Err(DbError::QueryError(e)),
    }?;

    match query!(
        r#"
        DELETE FROM users
        WHERE id = ?"#,
        id,
    )
    .execute(pool)
    .await
    {
        Ok(_) => Ok(()),
        Err(e) => Err(DbError::QueryError(e)),
    }
}

#[derive(Debug)]
pub struct Media {
    pub id: i64,
    pub r#type: MediaType,
    pub state: MediaState,
}

pub async fn get_media_by_user_id(pool: &SqlitePool, user_id: i64) -> Result<Vec<Media>, DbError> {
    query_as!(
        Media,
        r#"
SELECT id, type as "type: MediaType", state as "state: MediaState"
FROM media
WHERE user_id = ?"#,
        user_id,
    )
    .fetch_all(pool)
    .await
    .map_err(DbError::QueryError)
}

pub async fn get_specific_media_by_user_id(
    pool: &SqlitePool,
    media_type: MediaType,
    media_id: i64,
    user_id: i64,
) -> Result<Option<Media>, DbError> {
    query_as!(
        Media,
        r#"
SELECT id, type as "type: MediaType", state as "state: MediaState"
FROM media
WHERE type = ? AND id = ? AND user_id = ?"#,
        media_type,
        media_id,
        user_id,
    )
    .fetch_optional(pool)
    .await
    .map_err(DbError::QueryError)
}
