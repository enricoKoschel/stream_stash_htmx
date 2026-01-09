use serde::Deserialize;
use std::fmt::Display;

pub mod search;
pub mod tmdb;

pub const MEDIA_TYPES: [MediaType; 2] = [MediaType::Movies, MediaType::TvShows];

// TODO: Should be truly case insensitive
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
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
