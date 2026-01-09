use crate::AppState;
use crate::media_source::search::{SearchQuery, build_search_url, fetch_search_results};
use crate::views::components::{media_card, media_cards};
use crate::views::pages::{about_page, card_page};
use crate::views::{maybe_document, maybe_redirect};
use axum::extract::{Path, Query, State};
use axum::http::{StatusCode, Uri};
use axum::response::{IntoResponse, Response};
use axum::{Router, routing::get};
use axum_htmx::HxRequest;
use maud::Markup;
use std::iter;

async fn index(HxRequest(hx_request): HxRequest, path: Option<Path<usize>>) -> impl IntoResponse {
    let count = path.map_or(50, |p| p.0);
    let media = generate_sample_media(count);
    maybe_document(hx_request, card_page(None, &media))
}

async fn about(HxRequest(hx_request): HxRequest) -> impl IntoResponse {
    maybe_document(hx_request, about_page())
}

// TODO: Show total results somewhere
// TODO: Somehow show that last page was loaded
async fn search(
    HxRequest(hx_request): HxRequest,
    Query(query): Query<SearchQuery>,
    State(state): State<AppState>,
    uri: Uri,
) -> impl IntoResponse {
    if query.q.trim().is_empty() {
        return StatusCode::BAD_REQUEST.into_response();
    }

    let uri_str = uri.to_string();

    match query.p {
        Some(page) => handle_paginated_search(hx_request, &state, &query, &uri_str, page).await,
        None => handle_initial_search(hx_request, &state, &query, &uri_str).await,
    }
}

async fn handle_paginated_search(
    hx_request: bool,
    state: &AppState,
    query: &SearchQuery,
    uri_str: &str,
    page: i32,
) -> Response {
    // TODO: Redirect before doing all of this unnecessary work?
    let next_page_url = build_search_url(uri_str, &query.q, query.t, Some(page + 1));
    let cards =
        fetch_search_results(&state.tmdb_service, &query.q, query.t, page, &next_page_url).await;

    let url_no_page = build_search_url(uri_str, &query.q, query.t, None);
    maybe_redirect(hx_request, &url_no_page, media_cards(&cards))
}

async fn handle_initial_search(
    hx_request: bool,
    state: &AppState,
    query: &SearchQuery,
    uri_str: &str,
) -> Response {
    let next_page_url = build_search_url(uri_str, &query.q, query.t, Some(2));
    let cards =
        fetch_search_results(&state.tmdb_service, &query.q, query.t, 1, &next_page_url).await;
    maybe_document(hx_request, card_page(Some(query), &cards))
}

fn generate_sample_media(count: usize) -> Vec<Markup> {
    iter::repeat_with(|| [
        media_card(
            "Harry Potter and the Philosopher's Stone",
            "2001",
            Some("https://image.tmdb.org/t/p/w600_and_h900_bestv2/wuMc08IPKEatf9rnMNXvIDxqP4W.jpg"),
            "/media/movie/1",
            None,
        ),
        media_card(
            "Breaking Bad",
            "2008",
            Some("https://image.tmdb.org/t/p/w600_and_h900_bestv2/ztkUQFLlC19CCMYHW9o1zWhJRNq.jpg"),
            "/media/tv/1",
            None,
        ),
    ])
    .flatten()
    .take(count)
    .collect()
}

pub fn index_router() -> Router<AppState> {
    Router::new()
        .route("/", get(index))
        .route("/{count}", get(index))
        .route("/about", get(about))
        .route("/search", get(search))
}
