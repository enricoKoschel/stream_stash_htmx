use crate::AppState;
use crate::media_source::tmdb::TmdbService;
use crate::views::components::media_card;
use crate::views::maybe_document;
use crate::views::pages::{about_page, card_page};
use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use axum::{Router, routing::get};
use axum_htmx::HxRequest;
use maud::Markup;
use serde::Deserialize;
use std::iter;

async fn index(HxRequest(hx_request): HxRequest, path: Option<Path<usize>>) -> impl IntoResponse {
    let count = path.map_or(50, |p| p.0);

    let media: Vec<Markup> = iter::repeat_with(|| [
        media_card(
            "Harry Potter and the Philosopher's Stone",
            "2001",
            Some("https://image.tmdb.org/t/p/w600_and_h900_bestv2/wuMc08IPKEatf9rnMNXvIDxqP4W.jpg"),
            "/media/movie/1",
        ),
        media_card(
            "Breaking Bad",
            "2008",
            Some("https://image.tmdb.org/t/p/w600_and_h900_bestv2/ztkUQFLlC19CCMYHW9o1zWhJRNq.jpg"),
            "/media/tv/1",
        ),
    ]).flatten().take(count).collect();

    // TODO: Do search query with optional?
    maybe_document(hx_request, card_page(("", "Movies"), &media))
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
    // TODO: Error handling, better matching?
    let cards: Vec<Markup> = match search_query.t.as_str() {
        "Movies" => {
            let search_result = state
                .tmdb_service
                .search_movies(&search_query.q)
                .await
                .unwrap();

            search_result
                .results
                .into_iter()
                .map(|media| {
                    let title = media
                        .title
                        .as_deref()
                        .filter(|title| !title.is_empty())
                        .unwrap_or("????");
                    let year = &media
                        .release_date
                        .as_deref()
                        .filter(|date| !date.is_empty())
                        .unwrap_or("????")[0..4];
                    let poster_url = media
                        .poster_path
                        .and_then(|path| TmdbService::get_image_url(&path).ok())
                        .map(String::from);
                    let media_page_url = &format!("/media/movie/{}", media.id);

                    media_card(title, year, poster_url.as_deref(), media_page_url)
                })
                .collect()
        }
        "TV Shows" => {
            let search_result = state
                .tmdb_service
                .search_tv_shows(&search_query.q)
                .await
                .unwrap();

            search_result
                .results
                .into_iter()
                .map(|media| {
                    let name = media
                        .name
                        .as_deref()
                        .filter(|name| !name.is_empty())
                        .unwrap_or("????");
                    let year = &media
                        .first_air_date
                        .as_deref()
                        .filter(|date| !date.is_empty())
                        .unwrap_or("????")[0..4];
                    let poster_url = media
                        .poster_path
                        .and_then(|path| TmdbService::get_image_url(&path).ok())
                        .map(String::from);
                    let media_page_url = &format!("/media/tv/{}", media.id);

                    media_card(name, year, poster_url.as_deref(), media_page_url)
                })
                .collect()
        }
        _ => todo!(),
    };

    maybe_document(
        hx_request,
        card_page((&search_query.q, &search_query.t), &cards),
    )
}

pub fn index_router() -> Router<AppState> {
    Router::new()
        .route("/", get(index))
        .route("/{count}", get(index))
        .route("/about", get(about))
        .route("/search", get(search))
}
