use crate::AppState;
use crate::views::components::MediaCard;
use crate::views::maybe_document;
use crate::views::pages::{about_page, main_page, search_page};
use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use axum::{Router, routing::get};
use axum_htmx::HxRequest;
use serde::Deserialize;
use std::iter;

async fn index(HxRequest(hx_request): HxRequest, path: Option<Path<usize>>) -> impl IntoResponse {
    let count = path.map_or(50, |p| p.0);

    let media: Vec<MediaCard> = iter::repeat_with(|| [
        MediaCard {
            title: "Harry Potter and the Philosopher's Stone",
            year: "2001",
            poster_url: "https://image.tmdb.org/t/p/w600_and_h900_bestv2/wuMc08IPKEatf9rnMNXvIDxqP4W.jpg",
            media_page_url: "/media/movie/1",
        },
        MediaCard {
            title: "Breaking Bad",
            year: "2008",
            poster_url: "https://image.tmdb.org/t/p/w600_and_h900_bestv2/ztkUQFLlC19CCMYHW9o1zWhJRNq.jpg",
            media_page_url: "/media/tv/1",
        },
    ]).flatten().take(count).collect();

    maybe_document(hx_request, main_page(&media))
}

async fn about(HxRequest(hx_request): HxRequest) -> impl IntoResponse {
    maybe_document(hx_request, about_page())
}

#[derive(Deserialize)]
struct SearchQuery {
    q: String,
    t: String,
}

async fn search(
    HxRequest(hx_request): HxRequest,
    Query(search_query): Query<SearchQuery>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let search_result = match search_query.t.as_str() {
        "Movies" => state
            .tmdb_service
            .search_movies(&search_query.q)
            .await
            .unwrap(),
        "TV Shows" => todo!(),
        _ => todo!(),
    };
    maybe_document(
        hx_request,
        search_page(
            (&search_query.q, &search_query.t),
            &format!("{search_result:?}"),
        ),
    )
}

pub fn index_router() -> Router<AppState> {
    Router::new()
        .route("/", get(index))
        .route("/{count}", get(index))
        .route("/about", get(about))
        .route("/search", get(search))
}
