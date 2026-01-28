use crate::AppSession;
use crate::data_source::{Media, MediaType};
use crate::{AppState, views::maybe_document};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Router, extract::Path, routing::get};
use axum_htmx::HxRequest;

async fn media(
    HxRequest(hx_request): HxRequest,
    State(state): State<AppState>,
    Path((media_type, id)): Path<(MediaType, i32)>,
    _session: AppSession,
) -> impl IntoResponse {
    // TODO: Get db entry for media
    let media: Media = match media_type {
        MediaType::Movies => match state.tmdb_service.movie_details(id).await {
            Ok(movie) => Media::Movie(movie),
            Err(e) => {
                tracing::error!("Failed to fetch movie details for id {}: {}", id, e);
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to load movie details",
                )
                    .into_response();
            }
        },
        MediaType::TvShows => match state.tmdb_service.tv_show_details(id).await {
            Ok(tv_show) => Media::TvShow(tv_show),
            Err(e) => {
                tracing::error!("Failed to fetch TV show details for id {}: {}", id, e);
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to load TV show details",
                )
                    .into_response();
            }
        },
    };

    let page = state.tmdb_service.map_media_to_page(&media);
    maybe_document(hx_request, &state.google_client_id, state.login_url, page)
}

pub fn media_router() -> Router<AppState> {
    Router::new().route("/{media_type}/{id}", get(media))
}
