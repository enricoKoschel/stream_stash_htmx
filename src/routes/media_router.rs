use crate::data_source::MediaType;
use crate::data_source::tmdb::ImageType;
use crate::views::pages::media_page;
use crate::{AppState, views::maybe_document};
use axum::extract::State;
use axum::response::IntoResponse;
use axum::{Router, extract::Path, routing::get};
use axum_htmx::HxRequest;

async fn media(
    HxRequest(hx_request): HxRequest,
    State(state): State<AppState>,
    Path((media_type, id)): Path<(MediaType, i32)>,
) -> impl IntoResponse {
    // TODO: Get db entry for media
    let (title, overview, release_date, poster_path, backdrop_path) = match media_type {
        MediaType::Movies => {
            // TODO: Error handling!
            let details = state.tmdb_service.movie_details(id).await.unwrap();
            (
                details.title,
                details.overview,
                details.release_date,
                details.poster_path,
                details.backdrop_path,
            )
        }
        MediaType::TvShows => {
            // TODO: Error handling!
            let details = state.tmdb_service.tv_show_details(id).await.unwrap();
            (
                details.name,
                details.overview,
                details.first_air_date,
                details.poster_path,
                details.backdrop_path,
            )
        }
    };

    let title = title.as_deref().unwrap_or("????");
    let overview = overview.as_deref().unwrap_or_default();
    let release_date = release_date.as_deref().unwrap_or("????-??-??");
    let poster_url = poster_path
        .and_then(|path| {
            state
                .tmdb_service
                .get_image_url(&path, ImageType::Poster)
                .ok()
        })
        .map(String::from);
    let backdrop_url = backdrop_path
        .and_then(|path| {
            state
                .tmdb_service
                .get_image_url(&path, ImageType::Backdrop)
                .ok()
        })
        .map(String::from);

    maybe_document(
        hx_request,
        media_page(
            title,
            overview,
            release_date,
            poster_url.as_deref(),
            backdrop_url.as_deref(),
        ),
    )
}

pub fn media_router() -> Router<AppState> {
    Router::new().route("/{media_type}/{id}", get(media))
}
