use crate::views::pages::media_page;
use crate::{AppState, views::maybe_document};
use axum::body::Body;
use axum::http::Response;
use axum::{Router, extract::Path, routing::get};
use axum_htmx::HxRequest;

async fn media(
    HxRequest(hx_request): HxRequest,
    Path((media_type, id)): Path<(String, String)>,
) -> Response<Body> {
    // TODO: Get db entry for media, get tmdb entry for media, then display
    // TODO: Optional for search query?
    maybe_document(hx_request, media_page(("", "Movies"), &media_type, &id))
}

pub fn media_router() -> Router<AppState> {
    Router::new().route("/{media_type}/{id}", get(media))
}
