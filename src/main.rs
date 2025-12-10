use axum::{Router, routing::get};
use std::net::Ipv4Addr;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tracing_subscriber::filter::LevelFilter;

use crate::routes::index;

mod routes;
mod views;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: Get from env
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .init();

    let app = Router::new()
        .route_service("/", get(index))
        .nest_service("/static", ServeDir::new("./static"))
        // .route("/test", get(test))
        .layer(TraceLayer::new_for_http());

    let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 8000)).await?;

    axum::serve(listener, app).await?;

    Ok(())
}
