use serde::Deserialize;
use std::fmt::Display;

pub mod tmdb;

pub const MEDIA_TYPES: [MediaType; 2] = [MediaType::Movies, MediaType::TvShows];

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub enum MediaType {
    Movies,
    #[serde(rename = "TV Shows")]
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
