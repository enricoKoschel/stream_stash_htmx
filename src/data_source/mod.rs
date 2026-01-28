use serde::Deserialize;
use std::fmt::Display;

pub mod db;
pub mod google_auth;
pub mod search;
pub mod tmdb;

pub const MEDIA_TYPES: [MediaType; 2] = [MediaType::Movies, MediaType::TvShows];

#[derive(Debug)]
pub enum Media {
    Movie(tmdb::Movie),
    TvShow(tmdb::TvShow),
}

impl Media {
    pub fn id(&self) -> i32 {
        match self {
            Media::Movie(movie) => movie.id,
            Media::TvShow(tv_show) => tv_show.id,
        }
    }

    pub fn r#type(&self) -> MediaType {
        match self {
            Media::Movie(_) => MediaType::Movies,
            Media::TvShow(_) => MediaType::TvShows,
        }
    }

    pub fn title(&self) -> Option<&str> {
        match self {
            Media::Movie(movie) => movie.title.as_deref(),
            Media::TvShow(tv_show) => tv_show.name.as_deref(),
        }
    }

    pub fn overview(&self) -> Option<&str> {
        match self {
            Media::Movie(movie) => movie.overview.as_deref(),
            Media::TvShow(tv_show) => tv_show.overview.as_deref(),
        }
    }

    pub fn release_date(&self) -> Option<&str> {
        match self {
            Media::Movie(movie) => movie.release_date.as_deref(),
            Media::TvShow(tv_show) => tv_show.first_air_date.as_deref(),
        }
    }

    pub fn poster_path(&self) -> Option<&str> {
        match self {
            Media::Movie(movie) => movie.poster_path.as_deref(),
            Media::TvShow(tv_show) => tv_show.poster_path.as_deref(),
        }
    }

    pub fn backdrop_path(&self) -> Option<&str> {
        match self {
            Media::Movie(movie) => movie.backdrop_path.as_deref(),
            Media::TvShow(tv_show) => tv_show.backdrop_path.as_deref(),
        }
    }
}

// TODO: Should be truly case insensitive
#[derive(Debug, Clone, Copy, PartialEq, Deserialize, sqlx::Type)]
pub enum MediaType {
    #[serde(alias = "movies", alias = "MOVIES")]
    Movies,
    #[serde(
        rename = "TV Shows",
        alias = "tv shows",
        alias = "TV SHOWS",
        alias = "Tv Shows",
        alias = "Tv shows"
    )]
    TvShows,
}

impl Display for MediaType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Movies => write!(f, "Movies"),
            Self::TvShows => write!(f, "TV Shows"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize, sqlx::Type)]
pub enum MediaState {
    Done,
    InProgress,
    Todo,
    Waiting,
    Dropped,
}
