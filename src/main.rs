use crate::routes::index_router;
use axum::Router;
use axum_htmx::AutoVaryLayer;
use std::net::Ipv4Addr;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tracing_subscriber::filter::LevelFilter;

mod routes;
mod views;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: Get from env
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .init();

    let app = Router::new()
        .fallback_service(index_router())
        .nest_service("/static", ServeDir::new("./static"))
        .layer(TraceLayer::new_for_http())
        .layer(AutoVaryLayer);

    let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 8000)).await?;

    axum::serve(listener, app).await?;

    Ok(())
}
