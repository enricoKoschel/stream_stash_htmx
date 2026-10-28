use crate::data_source::google_auth::GoogleAuthService;
use crate::data_source::tmdb::TmdbService;
use crate::routes::{index_router, media_router};
use crate::static_files::static_router;
use axum::Router;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::response::Redirect;
use axum_cookie::CookieLayer;
use axum_htmx::AutoVaryLayer;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use sqlx::sqlite::SqliteConnectOptions;
use std::net::Ipv4Addr;
use std::str::FromStr;
use tokio::{signal, task::AbortHandle};
use tower_http::compression::CompressionLayer;
use tower_http::trace::TraceLayer;
use tower_sessions::cookie::SameSite;
use tower_sessions::{ExpiredDeletion, Session, SessionManagerLayer};
use tower_sessions_sqlx_store::SqliteStore;
use tracing_subscriber::filter::LevelFilter;

mod data_source;
mod routes;
mod static_files;
mod views;

#[derive(Debug, Deserialize, Serialize)]
pub struct AppSession {
    pub user_id: i64,
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

// Necessary to avoid the orphan rule
pub struct MaybeAppSession(pub Option<AppSession>);

impl<S> FromRequestParts<S> for MaybeAppSession
where
    S: Send + Sync,
{
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(req: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let session = Session::from_request_parts(req, state).await.ok();

        let app_session = match session {
            Some(session) => session.get("session").await.ok().flatten(),
            None => None,
        };

        Ok(MaybeAppSession(app_session))
    }
}

#[derive(Clone)]
struct AppState {
    tmdb_service: TmdbService,
    db_pool: SqlitePool,
    google_auth_service: GoogleAuthService,
    google_client_id: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(debug_assertions)]
    {
        if let Err(e) = dotenvy::dotenv() {
            panic!("Error while loading .env file: {e}")
        }
    }

    let tmdb_read_access_token = std::env::var("TMDB_READ_ACCESS_TOKEN")
        .expect("Please provide a TMDB_READ_ACCESS_TOKEN environment variable");
    let sqlite_connection_string =
        std::env::var("DATABASE_URL").expect("Please provide a DATABASE_URL environment variable");
    let google_client_id = std::env::var("GOOGLE_CLIENT_ID")
        .expect("Please provide a GOOGLE_CLIENT_ID environment variable");

    let (log_level, should_warn) = {
        match std::env::var("LOG_LEVEL") {
            Ok(log_level) => match LevelFilter::from_str(&log_level) {
                Ok(log_level) => (log_level, false),
                Err(_) => (LevelFilter::INFO, true),
            },
            Err(_) => (LevelFilter::INFO, true),
        }
    };
    tracing_subscriber::fmt().with_max_level(log_level).init();
    if should_warn {
        tracing::warn!(
            "No or invalid LOG_LEVEL environment variable found, using default value of INFO"
        );
    } else {
        tracing::info!("Current LOG_LEVEL: {}", log_level);
    }

    let connect_options =
        SqliteConnectOptions::from_str(&sqlite_connection_string)?.create_if_missing(true);
    let db_pool = SqlitePool::connect_with(connect_options).await?;

    sqlx::migrate!("./migrations").run(&db_pool).await?;
    tracing::info!("Database migrations applied");

    let session_store = SqliteStore::new(db_pool.clone());

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
        tmdb_service: TmdbService::new(&tmdb_read_access_token)?,
        db_pool,
        google_auth_service: GoogleAuthService::new(google_client_id.clone()),
        google_client_id,
    };

    let app = Router::new()
        .merge(index_router())
        .merge(static_router())
        .nest("/media", media_router())
        .layer(TraceLayer::new_for_http())
        .layer(AutoVaryLayer)
        .layer(CookieLayer::strict())
        .layer(session_layer)
        .layer(CompressionLayer::new())
        .with_state(app_state);

    #[cfg(debug_assertions)]
    {
        let listener = tokio::net::TcpListener::bind((Ipv4Addr::UNSPECIFIED, 8000)).await?;

        axum::serve(listener, app)
            .with_graceful_shutdown(shutdown_signal(deletion_task.abort_handle()))
            .await?;
    }

    #[cfg(not(debug_assertions))]
    {
        let tls_cert_path = std::env::var("TLS_CERT_PATH")
            .expect("Please provide a TLS_CERT_PATH environment variable");
        let tls_key_path = std::env::var("TLS_KEY_PATH")
            .expect("Please provide a TLS_KEY_PATH environment variable");

        let tls_config =
            axum_server::tls_rustls::RustlsConfig::from_pem_file(tls_cert_path, tls_key_path)
                .await?;

        let handle = axum_server::Handle::<std::net::SocketAddr>::new();
        let shutdown_handle = handle.clone();
        let abort_handle = deletion_task.abort_handle();
        tokio::spawn(async move {
            shutdown_signal(abort_handle).await;
            shutdown_handle.graceful_shutdown(None);
        });

        axum_server::bind_rustls((Ipv4Addr::UNSPECIFIED, 443).into(), tls_config)
            .handle(handle)
            .serve(app.into_make_service())
            .await?;
    }

    match deletion_task.await {
        Ok(result) => result?,
        Err(error) if error.is_cancelled() => {}
        Err(error) => return Err(error.into()),
    }

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
