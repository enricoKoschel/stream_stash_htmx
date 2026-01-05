use reqwest::{
    Client, Url,
    header::{self, HeaderMap, HeaderValue},
};

#[derive(Clone)]
pub struct TmdbService {
    client: Client,
    base_url: Url,
}

#[allow(unused)]
#[derive(Debug, serde::Deserialize)]
pub struct Media {
    pub adult: Option<bool>,
    pub backdrop_path: Option<String>,
    pub genre_ids: Option<Vec<i32>>,
    pub id: Option<i32>,
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

#[derive(Debug, serde::Deserialize)]
pub struct SearchResult {
    pub page: i32,
    pub total_pages: i32,
    pub total_results: i32,
    pub results: Vec<Media>,
}

impl TmdbService {
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

    pub async fn search_movies(&self, search_term: &str) -> Result<SearchResult, anyhow::Error> {
        #[derive(serde::Serialize)]
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
            page: None,
            region: None,
            year: None,
        };
        let url = self.base_url.join("/3/search/movie")?;

        let res = self.client.get(url).query(&query).send().await?;
        let json = res.json::<SearchResult>().await?;

        Ok(json)
    }
}
