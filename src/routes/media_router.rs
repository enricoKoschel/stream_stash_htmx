use crate::{AppState, views::maybe_document};
use axum::{Router, extract::Path, response::IntoResponse, routing::get};
use axum_htmx::HxRequest;

async fn test(
    HxRequest(hx_request): HxRequest,
    Path((media_type, id)): Path<(String, String)>,
) -> impl IntoResponse {
    // get db entry for media, get tmdb entry for media, then display
    maybe_document(hx_request, format!("{media_type}:{id}"))
}

pub fn media_router() -> Router<AppState> {
    Router::new().route("/{media_type}/{id}", get(test))
}
