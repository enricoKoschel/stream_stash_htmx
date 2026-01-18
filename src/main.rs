use crate::data_source::google_auth::GoogleAuthService;
use crate::data_source::tmdb::TmdbService;
use crate::routes::{index_router, media_router};
use axum::Router;
use axum_cookie::CookieLayer;
use axum_htmx::AutoVaryLayer;
use sqlx::SqlitePool;
use std::net::Ipv4Addr;
use tokio::net::TcpListener;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;
use tracing_subscriber::filter::LevelFilter;

mod data_source;
mod routes;
mod views;

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

    let app_state = AppState {
        tmdb_service: TmdbService::new(&tmdb_read_access_token),
        db_pool: SqlitePool::connect(&sqlite_connection_string).await?,
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
        .with_state(app_state);

    let listener = TcpListener::bind((Ipv4Addr::UNSPECIFIED, 8000)).await?;

    axum::serve(listener, app).await?;

    Ok(())
}
