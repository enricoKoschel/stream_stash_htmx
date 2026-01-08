use crate::AppState;
use crate::media_source::MediaType;
use crate::media_source::tmdb::TmdbService;
use crate::views::components::{media_card, media_cards};
use crate::views::pages::{about_page, card_page};
use crate::views::{maybe_document, maybe_redirect};
use axum::extract::{Path, Query, State};
use axum::http::{StatusCode, Uri};
use axum::response::IntoResponse;
use axum::{Router, routing::get};
use axum_htmx::HxRequest;
use maud::Markup;
use serde::Deserialize;
use std::iter;
use url::Url;

async fn index(HxRequest(hx_request): HxRequest, path: Option<Path<usize>>) -> impl IntoResponse {
    let count = path.map_or(50, |p| p.0);

    let media: Vec<Markup> = iter::repeat_with(|| [
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
    ]).flatten().take(count).collect();

    maybe_document(hx_request, card_page(None, &media))
}

async fn about(HxRequest(hx_request): HxRequest) -> impl IntoResponse {
    maybe_document(hx_request, about_page())
}

fn change_p_in_url(
    url: &str,
    search_term: &str,
    media_type: MediaType,
    new_p: Option<&str>,
) -> String {
    // "http://localhost" is only required to get the URL to parse,
    // relative URLs on their own do not parse
    // TODO: Do this better somehow?
    let base = Url::parse("http://localhost").expect("URL unparseable, should not happen");
    match base.join(url) {
        Ok(mut url) => {
            url.query_pairs_mut()
                .clear()
                .append_pair("q", search_term)
                .append_pair("t", &media_type.to_string());

            if let Some(new_p) = new_p {
                url.query_pairs_mut().append_pair("p", new_p);
            }

            base.make_relative(&url)
                .map_or("/".to_string(), |url| url.to_string())
        }
        Err(_) => "/".to_string(),
    }
}

#[derive(Deserialize)]
pub struct SearchQuery {
    #[serde(alias = "Q")]
    pub q: String,
    #[serde(alias = "T")]
    pub t: MediaType,
    #[serde(alias = "P")]
    pub p: Option<i32>,
}

// TODO: Clean all of the search code up
// TODO: Somehow show that all results were loaded
// TODO: Show total results somewhere
async fn search(
    HxRequest(hx_request): HxRequest,
    Query(search_query): Query<SearchQuery>,
    State(state): State<AppState>,
    uri: Uri,
) -> impl IntoResponse {
    if search_query.q.trim().is_empty() {
        return StatusCode::BAD_REQUEST.into_response();
    }

    match search_query.p {
        Some(page) => {
            let url_next_page = change_p_in_url(
                &uri.to_string(),
                &search_query.q,
                search_query.t,
                Some(&(page + 1).to_string()),
            );

            let cards: Vec<Markup> = match search_query.t {
                MediaType::Movies => {
                    search_movies(&state.tmdb_service, &search_query.q, page, &url_next_page).await
                }
                MediaType::TvShows => {
                    search_tv_shows(&state.tmdb_service, &search_query.q, page, &url_next_page)
                        .await
                }
            };

            let url_no_page =
                change_p_in_url(&uri.to_string(), &search_query.q, search_query.t, None);

            maybe_redirect(hx_request, &url_no_page, media_cards(&cards))
        }
        None => {
            let url_next_page =
                change_p_in_url(&uri.to_string(), &search_query.q, search_query.t, Some("2"));

            let cards: Vec<Markup> = match search_query.t {
                MediaType::Movies => {
                    search_movies(&state.tmdb_service, &search_query.q, 1, &url_next_page).await
                }
                MediaType::TvShows => {
                    search_tv_shows(&state.tmdb_service, &search_query.q, 1, &url_next_page).await
                }
            };

            maybe_document(hx_request, card_page(Some(&search_query), &cards))
        }
    }
}

async fn search_movies(
    tmdb_service: &TmdbService,
    search_term: &str,
    page: i32,
    next_page_url: &str,
) -> Vec<Markup> {
    // TODO: Error handling!
    let search_result = tmdb_service.search_movies(search_term, page).await.unwrap();

    let mut mapped = Vec::new();
    let mut it = search_result.results.into_iter().peekable();
    while let Some(media) = it.next() {
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

        let last_page = search_result.page >= search_result.total_pages;
        mapped.push(if it.peek().is_some() || last_page {
            media_card(title, year, poster_url.as_deref(), media_page_url, None)
        } else {
            media_card(
                title,
                year,
                poster_url.as_deref(),
                media_page_url,
                Some(next_page_url),
            )
        });
    }

    mapped
}

async fn search_tv_shows(
    tmdb_service: &TmdbService,
    search_term: &str,
    page: i32,
    next_page_url: &str,
) -> Vec<Markup> {
    // TODO: Error handling!
    let search_result = tmdb_service
        .search_tv_shows(search_term, page)
        .await
        .unwrap();

    let mut mapped = Vec::new();
    let mut it = search_result.results.into_iter().peekable();
    while let Some(media) = it.next() {
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

        let last_page = search_result.page >= search_result.total_pages;
        mapped.push(if it.peek().is_some() || last_page {
            media_card(name, year, poster_url.as_deref(), media_page_url, None)
        } else {
            media_card(
                name,
                year,
                poster_url.as_deref(),
                media_page_url,
                Some(next_page_url),
            )
        });
    }

    mapped
}

pub fn index_router() -> Router<AppState> {
    Router::new()
        .route("/", get(index))
        .route("/{count}", get(index))
        .route("/about", get(about))
        .route("/search", get(search))
}
