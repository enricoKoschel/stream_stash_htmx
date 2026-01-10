use reqwest::{
    Client,
    header::{self, HeaderMap, HeaderValue},
};
use serde::{Deserialize, Serialize};
use url::{ParseError, Url};

#[derive(Clone)]
pub struct TmdbService {
    client: Client,
    base_url: Url,
}

#[allow(unused)]
#[derive(Debug, Deserialize)]
pub struct Movie {
    pub adult: Option<bool>,
    pub backdrop_path: Option<String>,
    pub genre_ids: Option<Vec<i32>>,
    pub id: i32,
    pub original_language: Option<String>,
    pub original_title: Option<String>,
    pub overview: Option<String>,
    pub popularity: Option<f32>,
    pub poster_path: Option<String>,
    pub release_date: Option<String>,
    pub title: Option<String>,
    pub video: Option<bool>,
    pub vote_average: Option<f32>,
    pub vote_count: Option<i32>,
}

#[allow(unused)]
#[derive(Debug, Deserialize)]
pub struct TvShow {
    pub adult: Option<bool>,
    pub backdrop_path: Option<String>,
    pub genre_ids: Option<Vec<i32>>,
    pub id: i32,
    pub origin_country: Option<Vec<String>>,
    pub original_language: Option<String>,
    pub original_name: Option<String>,
    pub overview: Option<String>,
    pub popularity: Option<f32>,
    pub poster_path: Option<String>,
    pub first_air_date: Option<String>,
    pub name: Option<String>,
    pub vote_average: Option<f32>,
    pub vote_count: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct MovieSearchResult {
    pub page: i32,
    pub total_pages: i32,
    pub total_results: i32,
    pub results: Vec<Movie>,
}

#[derive(Debug, Deserialize)]
pub struct TvShowSearchResult {
    pub page: i32,
    pub total_pages: i32,
    pub total_results: i32,
    pub results: Vec<TvShow>,
}

impl TmdbService {
    pub fn get_image_url(image_path: &str) -> Result<Url, ParseError> {
        let base_url = Url::parse("https://image.tmdb.org/t/p/w600_and_h900_bestv2/")
            .expect("URL not parseable, should not happen");

        let image_path = image_path.strip_prefix('/').unwrap_or(image_path);

        base_url.join(image_path)
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
            base_url: Url::parse("https://api.themoviedb.org/")
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
        let url = self.base_url.join("/3/search/movie")?;

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
        let url = self.base_url.join("/3/search/tv")?;

        let res = self.client.get(url).query(&query).send().await?;
        let json = res.json::<TvShowSearchResult>().await?;

        Ok(json)
    }
}
