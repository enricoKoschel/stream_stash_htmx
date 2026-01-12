use reqwest::{
    Client,
    header::{self, HeaderMap, HeaderValue},
};
use serde::{Deserialize, Serialize};
use url::{ParseError, Url};

pub const ITEMS_PER_PAGE: i32 = 20;

#[derive(Clone)]
pub struct TmdbService {
    client: Client,
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

#[derive(Debug, Deserialize)]
pub struct MovieSearchResult {
    pub page: i32,
    pub total_pages: i32,
    pub total_results: i32,
    pub results: Vec<Movie>,
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

#[derive(Debug, Deserialize)]
pub struct TvShowSearchResult {
    pub page: i32,
    pub total_pages: i32,
    pub total_results: i32,
    pub results: Vec<TvShow>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ImageType {
    Poster,
    Backdrop,
}

impl TmdbService {
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

    pub fn new(tmdb_read_access_token: &str) -> Self {
        // TODO: Error handling here for all the expects in here?
        let mut auth_header = HeaderValue::from_str(&format!("Bearer {}", tmdb_read_access_token))
            .expect("tmdb_read_access_token cannot be parsed into an HTTP header");
        auth_header.set_sensitive(true);

        let mut headers = HeaderMap::new();
        headers.insert(header::AUTHORIZATION, auth_header);

        Self {
            client: Client::builder()
                .default_headers(headers)
                .build()
                .expect("Reqwest client could not be created for TmdbService"),
            api_base_url: Url::parse("https://api.themoviedb.org/")
                .expect("URL not parseable, should not happen"),
            poster_base_url: Url::parse("https://image.tmdb.org/t/p/w600_and_h900_bestv2/")
                .expect("URL not parseable, should not happen"),
            backdrop_base_url: Url::parse("https://image.tmdb.org/t/p/w1920_and_h1080_bestv2/")
                .expect("URL not parseable, should not happen"),
        }
    }

    pub async fn search_movies(
        &self,
        search_term: &str,
        page: i32,
    ) -> Result<MovieSearchResult, anyhow::Error> {
        // TODO: Remove this and use serde-json?
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

        let res = self.client.get(url).query(&query).send().await?;
        let json = res.json::<MovieSearchResult>().await?;

        Ok(json)
    }

    pub async fn search_tv_shows(
        &self,
        search_term: &str,
        page: i32,
    ) -> Result<TvShowSearchResult, anyhow::Error> {
        // TODO: Remove this and use serde-json?
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

        let res = self.client.get(url).query(&query).send().await?;
        let json = res.json::<TvShowSearchResult>().await?;

        Ok(json)
    }

    pub async fn movie_details(&self, movie_id: i32) -> Result<Movie, anyhow::Error> {
        let url = self.api_base_url.join(&format!("/3/movie/{movie_id}"))?;

        let res = self.client.get(url).send().await?;
        let json = res.json::<Movie>().await?;

        Ok(json)
    }

    pub async fn tv_show_details(&self, tv_show_id: i32) -> Result<TvShow, anyhow::Error> {
        let url = self.api_base_url.join(&format!("/3/tv/{tv_show_id}"))?;

        let res = self.client.get(url).send().await?;
        let json = res.json::<TvShow>().await?;

        Ok(json)
    }
}
