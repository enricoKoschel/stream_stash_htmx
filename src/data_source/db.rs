use serde::Deserialize;
use sqlx::{SqlitePool, query_as};
use thiserror::Error;
use time::OffsetDateTime;

#[derive(Debug, Error)]
pub enum DbError {
    #[error("Database query failed: {0}")]
    QueryError(#[from] sqlx::Error),
    #[error("User not found")]
    UserNotFound,
    #[error("User with this Google account ID already exists: {0}")]
    UserWithGoogleAccountIdAlreadyExists(String),
}

#[derive(Debug, Deserialize)]
pub struct User {
    pub id: i64,
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
    let user = query_as!(
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
            return DbError::UserWithGoogleAccountIdAlreadyExists(
                google_account_id.unwrap_or_default().to_string(),
            );
        }
        DbError::QueryError(e)
    })?;

    Ok(user)
}

pub async fn get_user_by_google_account_id(
    pool: &SqlitePool,
    google_account_id: &str,
) -> Result<User, DbError> {
    let user = query_as!(
        User,
        r#"
SELECT *
FROM users
WHERE google_account_id = ?"#,
        google_account_id,
    )
    .fetch_one(pool)
    .await
    .map_err(|e| {
        if let sqlx::Error::RowNotFound = &e {
            return DbError::UserNotFound;
        }

        DbError::QueryError(e)
    })?;

    Ok(user)
}
