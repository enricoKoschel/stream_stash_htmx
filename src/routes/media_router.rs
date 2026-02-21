use crate::AppSession;
use crate::data_source::db::{
    add_media_for_user, delete_media_for_user, get_specific_media_by_user_id,
    update_media_state_for_user,
};
use crate::data_source::{
    MEDIA_STATES_MOVIE, MEDIA_STATES_TV_SHOW, Media, MediaState, MediaType, TmdbMedia,
};
use crate::views::pages::media_page::{add_media_to_list_button, watch_state_dropdown};
use crate::{AppState, views::maybe_document};
use axum::Form;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Router, extract::Path, routing::get};
use axum_htmx::HxRequest;
use serde::Deserialize;

async fn get_media(
    HxRequest(hx_request): HxRequest,
    State(state): State<AppState>,
    Path((media_type, media_id)): Path<(MediaType, i64)>,
    session: AppSession,
) -> impl IntoResponse {
    let db_media =
        match get_specific_media_by_user_id(&state.db_pool, media_type, media_id, session.user_id)
            .await
        {
            Ok(db_media) => db_media,
            Err(e) => {
                tracing::error!(
                    "Failed to fetch media with type {} and id {} for user {}: {}",
                    media_type,
                    media_id,
                    session.user_id,
                    e
                );
                return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to load media").into_response();
            }
        };

    let media: Media = match media_type {
        MediaType::Movies => match state.tmdb_service.movie_details(media_id as i32).await {
            Ok(movie) => Media {
                tmdb_media: TmdbMedia::Movie(movie),
                state: db_media.map(|m| m.state),
            },
            Err(e) => {
                tracing::error!("Failed to fetch movie details for id {}: {}", media_id, e);
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to load movie details",
                )
                    .into_response();
            }
        },
        MediaType::TvShows => match state.tmdb_service.tv_show_details(media_id as i32).await {
            Ok(tv_show) => Media {
                tmdb_media: TmdbMedia::TvShow(tv_show),
                state: db_media.map(|m| m.state),
            },
            Err(e) => {
                tracing::error!("Failed to fetch TV show details for id {}: {}", media_id, e);
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to load TV show details",
                )
                    .into_response();
            }
        },
    };

    let title = Some(media.tmdb_media.sanitized_title());
    let page = state.tmdb_service.map_media_to_page(&media);
    maybe_document(
        hx_request,
        title.as_deref(),
        &state.google_client_id,
        state.login_url,
        page,
    )
}

async fn put_media(
    State(state): State<AppState>,
    Path((media_type, media_id)): Path<(MediaType, i64)>,
    session: AppSession,
) -> impl IntoResponse {
    let media =
        match add_media_for_user(&state.db_pool, media_type, media_id, session.user_id).await {
            Ok(media) => media,
            Err(e) => {
                tracing::error!(
                    "Failed to add media with type {} and id {} for user {}: {}",
                    media_type,
                    media_id,
                    session.user_id,
                    e
                );
                return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to add media").into_response();
            }
        };

    let states: Vec<_> = match media_type {
        MediaType::Movies => &MEDIA_STATES_MOVIE[..],
        MediaType::TvShows => &MEDIA_STATES_TV_SHOW[..],
    }
    .iter()
    .map(|s| s.to_string())
    .collect();

    watch_state_dropdown(
        &media.state.to_string(),
        &states.iter().map(|s| s.as_str()).collect::<Vec<_>>(),
        &format!("/media/{}/{}", media_type, media_id),
    )
    .into_response()
}

#[derive(Debug, Deserialize)]
struct MediaPatchBody {
    state: MediaState,
}

async fn patch_media(
    State(state): State<AppState>,
    Path((media_type, media_id)): Path<(MediaType, i64)>,
    session: AppSession,
    Form(body): Form<MediaPatchBody>,
) -> impl IntoResponse {
    match update_media_state_for_user(
        &state.db_pool,
        body.state,
        media_type,
        media_id,
        session.user_id,
    )
    .await
    {
        Ok(true) => StatusCode::NO_CONTENT.into_response(),
        Ok(false) => {
            tracing::info!(
                "Tried to patch state {} for media with type {} and id {} for user {}: Media does not exist",
                body.state,
                media_type,
                media_id,
                session.user_id,
            );
            (StatusCode::NOT_FOUND, "Media not found").into_response()
        }
        Err(e) => {
            tracing::error!(
                "Failed to patch state {} for media with type {} and id {} for user {}: {}",
                body.state,
                media_type,
                media_id,
                session.user_id,
                e
            );
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to patch media").into_response()
        }
    }
}

async fn delete_media(
    State(state): State<AppState>,
    Path((media_type, media_id)): Path<(MediaType, i64)>,
    session: AppSession,
) -> impl IntoResponse {
    if let Err(e) =
        delete_media_for_user(&state.db_pool, media_type, media_id, session.user_id).await
    {
        tracing::error!(
            "Failed to delete media with type {} and id {} for user {}: {}",
            media_type,
            media_id,
            session.user_id,
            e
        );
        return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to delete media").into_response();
    };

    add_media_to_list_button(&format!("/media/{}/{}", media_type, media_id)).into_response()
}

pub fn media_router() -> Router<AppState> {
    Router::new().route(
        "/{media_type}/{id}",
        get(get_media)
            .put(put_media)
            .patch(patch_media)
            .delete(delete_media),
    )
}
