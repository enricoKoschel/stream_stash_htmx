use crate::data_source::MediaType;
use crate::data_source::tmdb::TmdbService;
use crate::views::components::media_card;
use maud::Markup;
use serde::Deserialize;
use url::Url;

pub fn build_search_url(
    base_url: &str,
    query: &str,
    media_type: MediaType,
    page: Option<i32>,
) -> String {
    // TODO: Find a better approach for relative URL manipulation
    // "http://localhost" is necessary for the URL to parse
    // and is stripped out later
    let base = Url::parse("http://localhost").expect("base URL should parse");

    let Ok(mut url) = base.join(base_url) else {
        return "/".to_string();
    };

    {
        let mut pairs = url.query_pairs_mut();
        pairs
            .clear()
            .append_pair("q", query)
            .append_pair("t", &media_type.to_string());

        if let Some(p) = page {
            pairs.append_pair("p", &p.to_string());
        }
    }

    base.make_relative(&url).unwrap_or_else(|| "/".to_string())
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

pub async fn fetch_search_results(
    tmdb_service: &TmdbService,
    search_term: &str,
    media_type: MediaType,
    page: i32,
    next_page_url: &str,
) -> SearchResult {
    match media_type {
        MediaType::Movies => search_movies(tmdb_service, search_term, page, next_page_url).await,
        MediaType::TvShows => search_tv_shows(tmdb_service, search_term, page, next_page_url).await,
    }
}

pub struct SearchResult {
    pub cards: Vec<Markup>,
    pub shown_results: i32,
    pub total_results: i32,
}

async fn search_movies(
    tmdb_service: &TmdbService,
    search_term: &str,
    page: i32,
    next_page_url: &str,
) -> SearchResult {
    // TODO: Error handling!
    let result = tmdb_service.search_movies(search_term, page).await.unwrap();
    let is_last_page = result.page >= result.total_pages;

    let cards = map_to_media_cards(
        &result.results,
        |m| m.title.as_deref(),
        |m| m.release_date.as_deref(),
        |m| format!("/media/movie/{}", m.id),
        |m| m.poster_path.clone(),
        is_last_page,
        next_page_url,
    );

    // TODO: Actual shown results
    SearchResult {
        cards,
        shown_results: 0,
        total_results: result.total_results,
    }
}

async fn search_tv_shows(
    tmdb_service: &TmdbService,
    search_term: &str,
    page: i32,
    next_page_url: &str,
) -> SearchResult {
    // TODO: Error handling!
    let result = tmdb_service
        .search_tv_shows(search_term, page)
        .await
        .unwrap();
    let is_last_page = result.page >= result.total_pages;

    let cards = map_to_media_cards(
        &result.results,
        |m| m.name.as_deref(),
        |m| m.first_air_date.as_deref(),
        |m| format!("/media/tv/{}", m.id),
        |m| m.poster_path.clone(),
        is_last_page,
        next_page_url,
    );

    // TODO: Actual shown results
    SearchResult {
        cards,
        shown_results: 0,
        total_results: result.total_results,
    }
}

fn map_to_media_cards<T>(
    items: &[T],
    get_title: impl Fn(&T) -> Option<&str>,
    get_date: impl Fn(&T) -> Option<&str>,
    get_media_page_url: impl Fn(&T) -> String,
    get_poster_path: impl Fn(&T) -> Option<String>,
    is_last_page: bool,
    next_page_url: &str,
) -> Vec<Markup> {
    let mut cards = Vec::with_capacity(items.len());
    let mut it = items.iter().peekable();

    while let Some(item) = it.next() {
        let title = get_title(item)
            .filter(|s| !s.trim().is_empty())
            .unwrap_or("????");
        let year = get_date(item)
            .filter(|d| d.trim().len() >= 4)
            .map(|d| &d[0..4])
            .unwrap_or("????");
        let poster_url = get_poster_path(item)
            .and_then(|path| TmdbService::get_image_url(&path).ok())
            .map(String::from);
        let media_page_url = get_media_page_url(item);

        let is_last_item = it.peek().is_none();
        let load_more_url = if is_last_item && !is_last_page {
            Some(next_page_url)
        } else {
            None
        };

        cards.push(media_card(
            title,
            year,
            poster_url.as_deref(),
            &media_page_url,
            load_more_url,
        ));
    }

    cards
}
