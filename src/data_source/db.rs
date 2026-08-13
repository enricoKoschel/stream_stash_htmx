use crate::data_source::{DateValidity, MediaState, MediaType};
use serde::Deserialize;
use sqlx::{SqlitePool, query, query_as};
use thiserror::Error;
use time::{Date, PrimitiveDateTime};

#[derive(Debug, Error)]
pub enum DbError {
    #[error("Database query failed: {0}")]
    QueryError(#[from] sqlx::Error),
    #[error("User with this Google account ID already exists: {0}")]
    UserWithGoogleAccountIdAlreadyExists(String),
    #[error("Rating must be between 1 and 5, but is {0}")]
    RatingOutOfRange(i64),
}

#[derive(Debug, Deserialize)]
pub struct User {
    pub id: i64,
    #[allow(unused)]
    pub google_account_id: Option<String>,
    pub email: Option<String>,
    pub username: Option<String>,
    pub picture_url: Option<String>,
    pub created_at: PrimitiveDateTime,
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
RETURNING id, google_account_id, email, username, picture_url, created_at as "created_at: PrimitiveDateTime""#,
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
SELECT id, google_account_id, email, username, picture_url, created_at as "created_at: PrimitiveDateTime"
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
SELECT id, google_account_id, email, username, picture_url, created_at as "created_at: PrimitiveDateTime"
FROM users
WHERE id = ?"#,
        id,
    )
    .fetch_optional(pool)
    .await
    .map_err(DbError::QueryError)
}

pub async fn delete_user(pool: &SqlitePool, id: i64) -> Result<(), DbError> {
    query!(
        r#"
DELETE FROM media_history_entries
WHERE user_id = ?"#,
        id,
    )
    .execute(pool)
    .await
    .map_err(DbError::QueryError)?;

    query!(
        r#"
DELETE FROM media
WHERE user_id = ?"#,
        id,
    )
    .execute(pool)
    .await
    .map_err(DbError::QueryError)?;

    query!(
        r#"
DELETE FROM users
WHERE id = ?"#,
        id,
    )
    .execute(pool)
    .await
    .map(|_| ())
    .map_err(DbError::QueryError)
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

pub async fn create_or_replace_media_for_user(
    pool: &SqlitePool,
    media_type: MediaType,
    media_id: i64,
    user_id: i64,
) -> Result<Media, DbError> {
    query_as!(
        Media,
        r#"
INSERT OR REPLACE INTO media (type, id, user_id, state)
VALUES (?, ?, ?, ?)
RETURNING id, type as "type: MediaType", state as "state: MediaState""#,
        media_type,
        media_id,
        user_id,
        MediaState::Planned,
    )
    .fetch_one(pool)
    .await
    .map_err(DbError::QueryError)
}

pub async fn delete_media_for_user(
    pool: &SqlitePool,
    media_type: MediaType,
    media_id: i64,
    user_id: i64,
) -> Result<(), DbError> {
    query!(
        r#"
DELETE FROM media_history_entries
WHERE media_type = ? AND media_id = ? AND user_id = ?"#,
        media_type,
        media_id,
        user_id,
    )
    .execute(pool)
    .await
    .map(|_| ())
    .map_err(DbError::QueryError)?;

    query!(
        r#"
DELETE FROM media
WHERE type = ? AND id = ? AND user_id = ?"#,
        media_type,
        media_id,
        user_id,
    )
    .execute(pool)
    .await
    .map(|_| ())
    .map_err(DbError::QueryError)
}

pub async fn update_media_state_for_user(
    pool: &SqlitePool,
    media_state: MediaState,
    media_type: MediaType,
    media_id: i64,
    user_id: i64,
) -> Result<bool, DbError> {
    query!(
        r#"
UPDATE media
SET state = ?
WHERE type = ? AND id = ? AND user_id = ?"#,
        media_state,
        media_type,
        media_id,
        user_id,
    )
    .execute(pool)
    .await
    .map(|r| r.rows_affected() > 0)
    .map_err(DbError::QueryError)
}

#[derive(Debug, Deserialize)]
pub struct MediaHistoryEntry {
    pub id: i64,
    pub rating: Option<i64>,
    pub title: Option<String>,
    pub comment: Option<String>,
    pub start_date: Option<Date>,
    pub start_date_valid: DateValidity,
    pub end_date: Option<Date>,
    pub end_date_valid: DateValidity,
}

pub async fn get_media_history_entries_by_user_and_media(
    pool: &SqlitePool,
    user_id: i64,
    media_id: i64,
    media_type: MediaType,
) -> Result<Vec<MediaHistoryEntry>, DbError> {
    query_as!(
        MediaHistoryEntry,
        r#"
SELECT id, rating, title, comment,
       start_date, start_date_valid as "start_date_valid: DateValidity",
       end_date, end_date_valid as "end_date_valid: DateValidity"
FROM media_history_entries
WHERE user_id = ? AND media_id = ? AND media_type = ?
ORDER BY start_date DESC,
         CASE start_date_valid
             WHEN 'Everything' THEN 1
             WHEN 'YearAndMonth' THEN 2
             WHEN 'YearOnly' THEN 3
             ELSE 4
         END"#,
        user_id,
        media_id,
        media_type,
    )
    .fetch_all(pool)
    .await
    .map_err(DbError::QueryError)
}

#[allow(clippy::too_many_arguments)]
pub async fn create_media_history_entry_for_user_and_media(
    pool: &SqlitePool,
    user_id: i64,
    media_id: i64,
    media_type: MediaType,
    rating: Option<i64>,
    title: Option<String>,
    comment: Option<String>,
    start_date: Option<Date>,
    start_date_valid: DateValidity,
    end_date: Option<Date>,
    end_date_valid: DateValidity,
) -> Result<MediaHistoryEntry, DbError> {
    if let Some(rating) = rating
        && !(1..=5).contains(&rating)
    {
        return Err(DbError::RatingOutOfRange(rating));
    }

    query_as!(
        MediaHistoryEntry,
        r#"
INSERT INTO media_history_entries (user_id, media_id, media_type, rating, title,
            comment, start_date, start_date_valid, end_date, end_date_valid)
VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
RETURNING id, rating, title, comment,
          start_date, start_date_valid as "start_date_valid: DateValidity",
          end_date, end_date_valid as "end_date_valid: DateValidity""#,
        user_id,
        media_id,
        media_type,
        rating,
        title,
        comment,
        start_date,
        start_date_valid,
        end_date,
        end_date_valid,
    )
    .fetch_one(pool)
    .await
    .map_err(DbError::QueryError)
}

#[allow(clippy::too_many_arguments)]
pub async fn update_media_history_entry_for_user_by_id(
    pool: &SqlitePool,
    id: i64,
    user_id: i64,
    rating: Option<i64>,
    title: Option<String>,
    comment: Option<String>,
    start_date: Option<Date>,
    start_date_valid: DateValidity,
    end_date: Option<Date>,
    end_date_valid: DateValidity,
) -> Result<bool, DbError> {
    if let Some(rating) = rating
        && !(1..=5).contains(&rating)
    {
        return Err(DbError::RatingOutOfRange(rating));
    }

    query!(
        r#"
UPDATE media_history_entries
SET rating = ?, title = ?, comment = ?, start_date = ?, start_date_valid = ?, end_date = ?, end_date_valid = ? 
WHERE id = ? AND user_id = ?"#,
        rating,
        title,
        comment,
        start_date,
        start_date_valid,
        end_date,
        end_date_valid,
        id,
        user_id,
    )
    .execute(pool)
    .await
    .map(|r| r.rows_affected() > 0)
    .map_err(DbError::QueryError)
}

pub async fn delete_media_history_entry_for_user_by_id(
    pool: &SqlitePool,
    id: i64,
    user_id: i64,
) -> Result<(), DbError> {
    query!(
        r#"
DELETE FROM media_history_entries
WHERE id = ? AND user_id = ?"#,
        id,
        user_id,
    )
    .execute(pool)
    .await
    .map(|_| ())
    .map_err(DbError::QueryError)
}
