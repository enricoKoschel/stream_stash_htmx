use crate::views::pages::main_page;
use crate::views::{maybe_document, maybe_redirect};
use axum::response::IntoResponse;
use axum::{Router, routing::get};
use axum_htmx::HxRequest;

async fn index(HxRequest(hx_request): HxRequest) -> impl IntoResponse {
    maybe_document(hx_request, main_page())
}

pub fn index_router() -> Router {
    Router::<()>::new().route("/", get(index))
}
