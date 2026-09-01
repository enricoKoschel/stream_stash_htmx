use crate::AppSession;
use crate::data_source::db::{
    DATE_FORMAT, create_media_history_entry_for_user_and_media, create_or_replace_media_for_user,
    delete_media_history_entry_for_user_by_id, delete_specific_media_for_user,
    get_media_history_entries_by_user_and_media, get_specific_media_by_user_id,
    update_media_history_entry_for_user_by_id, update_media_state_for_user,
};
use crate::data_source::{
    MEDIA_STATES_MOVIE, MEDIA_STATES_TV_SHOW, Media, MediaState, MediaType, TmdbMedia,
};
use crate::views::components::{
    add_media_to_list_button, media_history_entry, watch_state_dropdown,
};
use crate::{AppState, views::maybe_document};
use axum::Form;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, patch, post};
use axum::{Router, extract::Path};
use axum_htmx::HxRequest;
use serde::Deserialize;
use time::Date;

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

    let history = match get_media_history_entries_by_user_and_media(
        &state.db_pool,
        session.user_id,
        media_id,
        media_type,
    )
    .await
    {
        Ok(history) => history,
        Err(e) => {
            tracing::error!(
                "Failed to fetch history for media with type {} and id {} for user {}: {}",
                media_type,
                media_id,
                session.user_id,
                e
            );
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to load history").into_response();
        }
    };

    let title = Some(media.tmdb_media.sanitized_title());
    let page = state.tmdb_service.map_media_to_page(&media, &history);
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
    let media = match create_or_replace_media_for_user(
        &state.db_pool,
        media_type,
        media_id,
        session.user_id,
    )
    .await
    {
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
        delete_specific_media_for_user(&state.db_pool, media_type, media_id, session.user_id).await
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

async fn post_media_history(
    State(state): State<AppState>,
    Path((media_type, media_id)): Path<(MediaType, i64)>,
    session: AppSession,
) -> impl IntoResponse {
    let history_entry = match create_media_history_entry_for_user_and_media(
        &state.db_pool,
        session.user_id,
        media_id,
        media_type,
        None,
        None,
        None,
        None,
        None,
    )
    .await
    {
        Ok(history_entry) => history_entry,
        Err(e) => {
            tracing::error!(
                "Failed to create media history entry for media with type {} and id {} for user {}: {}",
                media_type,
                media_id,
                session.user_id,
                e
            );
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to create media history entry",
            )
                .into_response();
        }
    };

    media_history_entry(
        &history_entry,
        &format!("/media/{media_type}/{media_id}/history"),
        media_type,
    )
    .into_response()
}

#[derive(Debug, Deserialize)]
struct MediaHistoryPatchBody {
    rating: Option<i64>,
    title: Option<String>,
    comment: Option<String>,
    start_date: Option<String>,
    end_date: Option<String>,
}

fn parse_date(date_str: Option<&str>) -> Result<Option<Date>, String> {
    match date_str
        .filter(|date_str| !date_str.is_empty())
        .map(|date_str| Date::parse(date_str, DATE_FORMAT))
    {
        Some(Ok(date)) => Ok(Some(date)),
        Some(Err(e)) => Err(e.to_string()),
        None => Ok(None),
    }
}

async fn patch_media_history_entry(
    State(state): State<AppState>,
    Path((media_type, media_id, entry_id)): Path<(MediaType, i64, i64)>,
    session: AppSession,
    Form(body): Form<MediaHistoryPatchBody>,
) -> impl IntoResponse {
    let Ok(start_date) = parse_date(body.start_date.as_deref()) else {
        return (StatusCode::BAD_REQUEST, "Invalid start_date").into_response();
    };
    let Ok(end_date) = parse_date(body.end_date.as_deref()) else {
        return (StatusCode::BAD_REQUEST, "Invalid end_date").into_response();
    };

    match update_media_history_entry_for_user_by_id(
        &state.db_pool,
        entry_id,
        media_id,
        media_type,
        session.user_id,
        body.rating,
        body.title.as_deref().map(|title| title.trim()),
        body.comment.as_deref().map(|comment| comment.trim()),
        start_date.as_ref(),
        end_date.as_ref(),
    )
    .await
    {
        Ok(true) => StatusCode::NO_CONTENT.into_response(),
        Ok(false) => {
            tracing::error!(
                "Failed to update media history entry {} for media with type {} and id {} for user {}: Media history entry not found",
                entry_id,
                media_type,
                media_id,
                session.user_id,
            );
            (
                StatusCode::NOT_FOUND,
                "Failed to update media history entry",
            )
                .into_response()
        }
        Err(e) => {
            tracing::error!(
                "Failed to update media history entry {} for media with type {} and id {} for user {}: {}",
                entry_id,
                media_type,
                media_id,
                session.user_id,
                e
            );
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to update media history entry",
            )
                .into_response()
        }
    }
}

async fn delete_media_history_entry(
    State(state): State<AppState>,
    Path((media_type, media_id, entry_id)): Path<(MediaType, i64, i64)>,
    session: AppSession,
) -> impl IntoResponse {
    if let Err(e) = delete_media_history_entry_for_user_by_id(
        &state.db_pool,
        entry_id,
        media_id,
        media_type,
        session.user_id,
    )
    .await
    {
        tracing::error!(
            "Failed to delete media history entry {} for media with type {} and id {} for user {}: {}",
            entry_id,
            media_type,
            media_id,
            session.user_id,
            e
        );
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to delete media history entry",
        )
            .into_response();
    }

    // OK instead of NO_CONTENT to clear the history entry on the page
    StatusCode::OK.into_response()
}

pub fn media_router() -> Router<AppState> {
    Router::new()
        .route(
            "/{media_type}/{media_id}",
            get(get_media)
                .put(put_media)
                .patch(patch_media)
                .delete(delete_media),
        )
        .route("/{media_type}/{media_id}/history", post(post_media_history))
        .route(
            "/{media_type}/{media_id}/history/{entry_id}",
            patch(patch_media_history_entry).delete(delete_media_history_entry),
        )
}
