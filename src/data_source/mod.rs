use serde::Deserialize;
use std::fmt::Display;

pub mod db;
pub mod google_auth;
pub mod search;
pub mod tmdb;

pub const MEDIA_TYPES: [MediaType; 2] = [MediaType::Movies, MediaType::TvShows];

pub const MEDIA_STATES_MOVIE: [MediaState; 3] = [
    MediaState::Finished,
    MediaState::Planned,
    MediaState::Dropped,
];
pub const MEDIA_STATES_TV_SHOW: [MediaState; 5] = [
    MediaState::Finished,
    MediaState::InProgress,
    MediaState::Planned,
    MediaState::Waiting,
    MediaState::Dropped,
];

#[derive(Debug)]
pub struct Media {
    pub tmdb_media: TmdbMedia,
    pub state: Option<MediaState>,
}

#[derive(Debug)]
pub enum TmdbMedia {
    Movie(tmdb::Movie),
    TvShow(tmdb::TvShow),
}

impl TmdbMedia {
    pub fn id(&self) -> i32 {
        match self {
            Self::Movie(movie) => movie.id,
            Self::TvShow(tv_show) => tv_show.id,
        }
    }

    pub fn r#type(&self) -> MediaType {
        match self {
            Self::Movie(_) => MediaType::Movies,
            Self::TvShow(_) => MediaType::TvShows,
        }
    }

    pub fn title(&self) -> Option<&str> {
        match self {
            Self::Movie(movie) => movie.title.as_deref(),
            Self::TvShow(tv_show) => tv_show.name.as_deref(),
        }
    }

    pub fn overview(&self) -> Option<&str> {
        match self {
            Self::Movie(movie) => movie.overview.as_deref(),
            Self::TvShow(tv_show) => tv_show.overview.as_deref(),
        }
    }

    pub fn release_date(&self) -> Option<&str> {
        match self {
            Self::Movie(movie) => movie.release_date.as_deref(),
            Self::TvShow(tv_show) => tv_show.first_air_date.as_deref(),
        }
    }

    pub fn poster_path(&self) -> Option<&str> {
        match self {
            Self::Movie(movie) => movie.poster_path.as_deref(),
            Self::TvShow(tv_show) => tv_show.poster_path.as_deref(),
        }
    }

    pub fn backdrop_path(&self) -> Option<&str> {
        match self {
            Self::Movie(movie) => movie.backdrop_path.as_deref(),
            Self::TvShow(tv_show) => tv_show.backdrop_path.as_deref(),
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
    Finished,
    InProgress,
    Planned,
    Waiting,
    Dropped,
}

impl Display for MediaState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Finished => write!(f, "Finished"),
            Self::InProgress => write!(f, "In progress"),
            Self::Planned => write!(f, "Planned"),
            Self::Waiting => write!(f, "Waiting"),
            Self::Dropped => write!(f, "Dropped"),
        }
    }
}
