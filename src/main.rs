use crate::data_source::google_auth::GoogleAuthService;
use crate::data_source::tmdb::TmdbService;
use crate::routes::{index_router, media_router};
use axum::Router;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::response::Redirect;
use axum_cookie::CookieLayer;
use axum_htmx::AutoVaryLayer;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::net::Ipv4Addr;
use tokio::net::TcpListener;
use tokio::{signal, task::AbortHandle};
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;
use tower_sessions::cookie::SameSite;
use tower_sessions::{ExpiredDeletion, Session, SessionManagerLayer};
use tower_sessions_sqlx_store::SqliteStore;
use tracing_subscriber::filter::LevelFilter;

mod data_source;
mod routes;
mod views;

#[derive(Debug, Deserialize, Serialize)]
pub struct AppSession {
    pub account_id: i64,
}

impl<S> FromRequestParts<S> for AppSession
where
    S: Send + Sync,
{
    type Rejection = Redirect;

    async fn from_request_parts(req: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // TODO: Redirect to original page after login
        let session = Session::from_request_parts(req, state)
            .await
            .map_err(|_| Redirect::to("/login"))?;

        let app_session: Option<AppSession> = session.get("session").await.ok().flatten();

        app_session.ok_or_else(|| Redirect::to("/login"))
    }
}

#[derive(Clone)]
struct AppState {
    tmdb_service: TmdbService,
    db_pool: SqlitePool,
    google_auth_service: GoogleAuthService,
    google_client_id: String,
    login_url: &'static str,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Err(e) = dotenvy::dotenv() {
        panic!("Error while loading .env file: {e}")
    }

    let tmdb_read_access_token = std::env::var("TMDB_READ_ACCESS_TOKEN")
        .expect("Please provide a TMDB_READ_ACCESS_TOKEN environment variable");
    let sqlite_connection_string =
        std::env::var("DATABASE_URL").expect("Please provide a DATABASE_URL environment variable");
    let google_client_id = std::env::var("GOOGLE_CLIENT_ID")
        .expect("Please provide a GOOGLE_CLIENT_ID environment variable");

    // TODO: Get from env
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .init();

    let db_pool = SqlitePool::connect(&sqlite_connection_string).await?;
    let session_store = SqliteStore::new(db_pool.clone());
    session_store.migrate().await?;

    let deletion_task = tokio::task::spawn(
        session_store
            .clone()
            .continuously_delete_expired(tokio::time::Duration::from_mins(5)),
    );

    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(!cfg!(debug_assertions))
        .with_same_site(SameSite::Strict)
        .with_http_only(true);

    let app_state = AppState {
        tmdb_service: TmdbService::new(&tmdb_read_access_token),
        db_pool,
        google_auth_service: GoogleAuthService::new(google_client_id.clone()),
        google_client_id,
        login_url: if cfg!(debug_assertions) {
            "http://localhost:8000/login"
        } else {
            "https://www.stream-stash.com/login"
        },
    };

    let app = Router::new()
        .merge(index_router())
        .route_service("/favicon.ico", ServeFile::new("./static/favicon.ico"))
        .nest_service("/static", ServeDir::new("./static"))
        .nest("/media", media_router())
        .layer(TraceLayer::new_for_http())
        .layer(AutoVaryLayer)
        .layer(CookieLayer::strict())
        .layer(session_layer)
        .with_state(app_state);

    let listener = TcpListener::bind((Ipv4Addr::UNSPECIFIED, 8000)).await?;

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal(deletion_task.abort_handle()))
        .await?;

    deletion_task.await??;

    Ok(())
}

async fn shutdown_signal(deletion_task_abort_handle: AbortHandle) {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => { deletion_task_abort_handle.abort() },
        _ = terminate => { deletion_task_abort_handle.abort() },
    }
}
