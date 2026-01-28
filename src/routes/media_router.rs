use crate::AppSession;
use crate::data_source::{Media, MediaType};
use crate::{AppState, views::maybe_document};
use axum::extract::State;
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
        // TODO: Error handling!
        MediaType::Movies => Media::Movie(state.tmdb_service.movie_details(id).await.unwrap()),
        MediaType::TvShows => Media::TvShow(state.tmdb_service.tv_show_details(id).await.unwrap()),
    };

    let page = state.tmdb_service.map_media_to_page(&media);
    maybe_document(hx_request, &state.google_client_id, state.login_url, page)
}

pub fn media_router() -> Router<AppState> {
    Router::new().route("/{media_type}/{id}", get(media))
}
