use crate::AppSession;
use crate::data_source::db::get_specific_media_by_user_id;
use crate::data_source::{Media, MediaType, TmdbMedia};
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
    session: AppSession,
) -> impl IntoResponse {
    let db_media =
        match get_specific_media_by_user_id(&state.db_pool, media_type, id as i64, session.user_id)
            .await
        {
            Ok(db_media) => db_media,
            Err(e) => {
                tracing::error!(
                    "Failed to fetch media with type {} and id {} for user {}: {}",
                    media_type,
                    id,
                    session.user_id,
                    e
                );
                return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to load media").into_response();
            }
        };

    let media: Media = match media_type {
        MediaType::Movies => match state.tmdb_service.movie_details(id).await {
            Ok(movie) => Media {
                tmdb_media: TmdbMedia::Movie(movie),
                state: db_media.map(|m| m.state),
            },
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
            Ok(tv_show) => Media {
                tmdb_media: TmdbMedia::TvShow(tv_show),
                state: db_media.map(|m| m.state),
            },
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
