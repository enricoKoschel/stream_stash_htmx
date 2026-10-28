use axum::Router;
use axum::extract::Path;
use axum::http::{HeaderMap, StatusCode, header};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use rust_embed::RustEmbed;

// Embedded into the binary in release builds, read from disk in debug builds
#[derive(RustEmbed)]
#[folder = "static/"]
struct StaticFiles;

pub fn static_router<S: Clone + Send + Sync + 'static>() -> Router<S> {
    Router::new()
        .route("/favicon.ico", get(|headers| serve("favicon.ico", headers)))
        .route(
            "/static/{*path}",
            get(|Path(path): Path<String>, headers| async move { serve(&path, headers).await }),
        )
}

async fn serve(path: &str, headers: HeaderMap) -> Response {
    let Some(file) = StaticFiles::get(path) else {
        return StatusCode::NOT_FOUND.into_response();
    };

    let etag = format!("\"{}\"", hex(&file.metadata.sha256_hash()));
    let cache_headers = [
        (header::ETAG, etag.clone()),
        // Files are not fingerprinted, so clients must revalidate
        (header::CACHE_CONTROL, "no-cache".to_string()),
    ];

    let not_modified = headers
        .get(header::IF_NONE_MATCH)
        .and_then(|value| value.to_str().ok())
        .is_some_and(|value| value.split(',').any(|tag| tag.trim() == etag));

    if not_modified {
        return (StatusCode::NOT_MODIFIED, cache_headers).into_response();
    }

    (
        cache_headers,
        [(header::CONTENT_TYPE, file.metadata.mimetype().to_string())],
        file.data,
    )
        .into_response()
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}
