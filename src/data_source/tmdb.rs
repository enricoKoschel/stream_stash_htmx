use maud::Markup;
use reqwest::{
    Client,
    header::{self, HeaderMap, HeaderValue},
};
use serde::{Deserialize, Serialize};
use url::{ParseError, Url};

use crate::{
    data_source::{MEDIA_STATES_MOVIE, MEDIA_STATES_TV_SHOW, Media, MediaType, TmdbMedia},
    views::{components::media_card, pages::media_page},
};

pub const ITEMS_PER_PAGE: i32 = 20;

#[derive(Clone)]
pub struct TmdbService {
    http_client: Client,
    api_base_url: Url,
    poster_base_url: Url,
    backdrop_base_url: Url,
}

// TODO: Other fields?
#[derive(Debug, Deserialize)]
pub struct Movie {
    pub backdrop_path: Option<String>,
    pub id: i32,
    pub overview: Option<String>,
    pub poster_path: Option<String>,
    pub release_date: Option<String>,
    pub title: Option<String>,
}

// TODO: Other fields?
#[derive(Debug, Deserialize)]
pub struct TvShow {
    pub backdrop_path: Option<String>,
    pub first_air_date: Option<String>,
    pub id: i32,
    pub name: Option<String>,
    pub overview: Option<String>,
    pub poster_path: Option<String>,
}

#[derive(Debug)]
pub struct SearchResult {
    pub page: i32,
    pub total_pages: i32,
    pub total_results: i32,
    pub results: Vec<TmdbMedia>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ImageType {
    Poster,
    Backdrop,
}

impl TmdbService {
    pub fn new(tmdb_read_access_token: &str) -> Result<Self, anyhow::Error> {
        let mut auth_header = HeaderValue::from_str(&format!("Bearer {}", tmdb_read_access_token))
            .map_err(|e| anyhow::anyhow!("Invalid TMDB access token: {}", e))?;
        auth_header.set_sensitive(true);

        let mut headers = HeaderMap::new();
        headers.insert(header::AUTHORIZATION, auth_header);

        let http_client = Client::builder()
            .default_headers(headers)
            .build()
            .map_err(|e| anyhow::anyhow!("Failed to create HTTP client: {}", e))?;

        Ok(Self {
            http_client,
            api_base_url: Url::parse("https://api.themoviedb.org/")?,
            poster_base_url: Url::parse("https://image.tmdb.org/t/p/w600_and_h900_bestv2/")?,
            backdrop_base_url: Url::parse("https://image.tmdb.org/t/p/w1920_and_h1080_bestv2/")?,
        })
    }

    pub fn get_image_url(
        &self,
        image_path: &str,
        image_type: ImageType,
    ) -> Result<Url, ParseError> {
        let image_path = image_path.strip_prefix('/').unwrap_or(image_path);

        match image_type {
            ImageType::Poster => self.poster_base_url.join(image_path),
            ImageType::Backdrop => self.backdrop_base_url.join(image_path),
        }
    }

    pub async fn search_movies(
        &self,
        search_term: &str,
        page: i32,
    ) -> Result<SearchResult, anyhow::Error> {
        #[derive(Serialize)]
        struct QueryParams<'a> {
            query: &'a str,
            include_adult: Option<bool>,
            language: Option<&'a str>,
            primary_release_year: Option<&'a str>,
            page: Option<i32>,
            region: Option<&'a str>,
            year: Option<&'a str>,
        }

        let query = QueryParams {
            query: search_term,
            include_adult: None,
            language: None,
            primary_release_year: None,
            page: Some(page),
            region: None,
            year: None,
        };
        let url = self.api_base_url.join("/3/search/movie")?;

        #[derive(Deserialize)]
        struct ApiResponse {
            page: i32,
            total_pages: i32,
            total_results: i32,
            results: Vec<Movie>,
        }

        let response = self
            .http_client
            .get(url)
            .query(&query)
            .send()
            .await?
            .error_for_status()?
            .json::<ApiResponse>()
            .await?;

        Ok(SearchResult {
            page: response.page,
            total_pages: response.total_pages,
            total_results: response.total_results,
            results: response.results.into_iter().map(TmdbMedia::Movie).collect(),
        })
    }

    pub async fn search_tv_shows(
        &self,
        search_term: &str,
        page: i32,
    ) -> Result<SearchResult, anyhow::Error> {
        #[derive(Serialize)]
        struct QueryParams<'a> {
            query: &'a str,
            first_air_date_year: Option<i32>,
            include_adult: Option<bool>,
            language: Option<&'a str>,
            page: Option<i32>,
            year: Option<&'a str>,
        }

        let query = QueryParams {
            query: search_term,
            first_air_date_year: None,
            include_adult: None,
            language: None,
            page: Some(page),
            year: None,
        };
        let url = self.api_base_url.join("/3/search/tv")?;

        #[derive(Deserialize)]
        struct ApiResponse {
            page: i32,
            total_pages: i32,
            total_results: i32,
            results: Vec<TvShow>,
        }

        let response = self
            .http_client
            .get(url)
            .query(&query)
            .send()
            .await?
            .error_for_status()?
            .json::<ApiResponse>()
            .await?;

        Ok(SearchResult {
            page: response.page,
            total_pages: response.total_pages,
            total_results: response.total_results,
            results: response
                .results
                .into_iter()
                .map(TmdbMedia::TvShow)
                .collect(),
        })
    }

    pub async fn movie_details(&self, movie_id: i32) -> Result<Movie, anyhow::Error> {
        let url = self.api_base_url.join(&format!("/3/movie/{movie_id}"))?;

        let movie = self
            .http_client
            .get(url)
            .send()
            .await?
            .error_for_status()?
            .json::<Movie>()
            .await?;

        Ok(movie)
    }

    pub async fn tv_show_details(&self, tv_show_id: i32) -> Result<TvShow, anyhow::Error> {
        let url = self.api_base_url.join(&format!("/3/tv/{tv_show_id}"))?;

        let tv_show = self
            .http_client
            .get(url)
            .send()
            .await?
            .error_for_status()?
            .json::<TvShow>()
            .await?;

        Ok(tv_show)
    }

    fn get_string_or_default<'a>(value: Option<&'a str>, default: &'a str) -> &'a str {
        value.filter(|s| !s.trim().is_empty()).unwrap_or(default)
    }

    fn get_image_url_string(
        &self,
        image_path: Option<&str>,
        image_type: ImageType,
    ) -> Option<String> {
        image_path
            .filter(|s| !s.trim().is_empty())
            .and_then(|path| self.get_image_url(path, image_type).ok())
            .map(String::from)
    }

    pub fn map_media_to_card(&self, media: &TmdbMedia, load_more_url: Option<&str>) -> Markup {
        let title = Self::get_string_or_default(media.title(), "????");
        let year = media
            .release_date()
            .filter(|d| d.trim().len() >= 4)
            .map(|d| &d[0..4])
            .unwrap_or("????");
        let poster_url = self.get_image_url_string(media.poster_path(), ImageType::Poster);
        let media_page_url = format!("/media/{}/{}", media.r#type(), media.id());

        media_card(
            title,
            year,
            poster_url.as_deref(),
            &media_page_url,
            load_more_url,
        )
    }

    pub fn map_media_to_page(&self, media: &Media) -> Markup {
        let title = Self::get_string_or_default(media.tmdb_media.title(), "????");
        let overview = Self::get_string_or_default(media.tmdb_media.overview(), "");
        let release_date =
            Self::get_string_or_default(media.tmdb_media.release_date(), "????-??-??");
        let poster_url =
            self.get_image_url_string(media.tmdb_media.poster_path(), ImageType::Poster);
        let backdrop_url =
            self.get_image_url_string(media.tmdb_media.backdrop_path(), ImageType::Backdrop);

        let states: Vec<_> = match media.tmdb_media.r#type() {
            MediaType::Movies => &MEDIA_STATES_MOVIE[..],
            MediaType::TvShows => &MEDIA_STATES_TV_SHOW[..],
        }
        .iter()
        .map(|s| s.to_string())
        .collect();

        media_page(
            title,
            overview,
            release_date,
            poster_url.as_deref(),
            backdrop_url.as_deref(),
            media.state.map(|s| s.to_string()).as_deref(),
            &states.iter().map(|s| s.as_str()).collect::<Vec<_>>(),
            &format!(
                "/media/{}/{}",
                media.tmdb_media.r#type(),
                media.tmdb_media.id()
            ),
        )
    }
}
