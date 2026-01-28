use crate::data_source::db::{
    DbError, create_user, delete_user, get_media_by_user_id, get_user_by_google_account_id,
    get_user_by_id,
};
use crate::data_source::search::{SearchQuery, build_search_url, fetch_search_results};
use crate::data_source::tmdb::TmdbService;
use crate::data_source::{Media, MediaType};
use crate::views::components::{card_collection, search_results_count_bar};
use crate::views::pages::{
    about_page, login_page, main_page, privacy_page, profile_page, search_page,
};
use crate::views::{maybe_document, maybe_redirect};
use crate::{AppSession, AppState};
use axum::Form;
use axum::Router;
use axum::extract::{Query, State};
use axum::http::{StatusCode, Uri};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum_cookie::CookieManager;
use axum_htmx::HxRequest;
use serde::Deserialize;
use time::format_description::well_known::Rfc2822;
use tower_sessions::Session;

async fn index(
    HxRequest(hx_request): HxRequest,
    State(state): State<AppState>,
    session: AppSession,
) -> impl IntoResponse {
    // TODO: Error handling!
    let media = get_media_by_user_id(&state.db_pool, session.user_id)
        .await
        .unwrap();

    let mut cards = Vec::with_capacity(media.len());
    for m in &media {
        let media = match m.r#type {
            // TODO: Error handling!
            MediaType::Movies => {
                Media::Movie(state.tmdb_service.movie_details(m.id as i32).await.unwrap())
            }
            MediaType::TvShows => Media::TvShow(
                state
                    .tmdb_service
                    .tv_show_details(m.id as i32)
                    .await
                    .unwrap(),
            ),
        };

        cards.push(state.tmdb_service.map_media_to_card(&media, None));
    }

    maybe_document(
        hx_request,
        &state.google_client_id,
        state.login_url,
        main_page(&cards),
    )
}

async fn about(
    HxRequest(hx_request): HxRequest,
    State(state): State<AppState>,
) -> impl IntoResponse {
    maybe_document(
        hx_request,
        &state.google_client_id,
        state.login_url,
        about_page(),
    )
}

async fn privacy(
    HxRequest(hx_request): HxRequest,
    State(state): State<AppState>,
) -> impl IntoResponse {
    maybe_document(
        hx_request,
        &state.google_client_id,
        state.login_url,
        privacy_page(),
    )
}

async fn search(
    HxRequest(hx_request): HxRequest,
    Query(query): Query<SearchQuery>,
    State(state): State<AppState>,
    _session: AppSession,
    uri: Uri,
) -> impl IntoResponse {
    if query.q.trim().is_empty() {
        return StatusCode::BAD_REQUEST.into_response();
    }

    let uri_str = uri.to_string();

    match query.p {
        Some(page) => {
            handle_paginated_search(hx_request, &state.tmdb_service, &query, &uri_str, page).await
        }
        None => {
            handle_initial_search(
                hx_request,
                &state.google_client_id,
                state.login_url,
                &state.tmdb_service,
                &query,
                &uri_str,
            )
            .await
        }
    }
}

async fn handle_paginated_search(
    hx_request: bool,
    tmdb_service: &TmdbService,
    query: &SearchQuery,
    uri_str: &str,
    page: i32,
) -> Response {
    // TODO: Redirect before doing all of this unnecessary work?
    let next_page_url = build_search_url(uri_str, &query.q, query.t, Some(page + 1));
    let result = fetch_search_results(tmdb_service, &query.q, query.t, page, &next_page_url).await;

    let url_no_page = build_search_url(uri_str, &query.q, query.t, None);
    let card_collection = card_collection(&result.cards, false, true);
    maybe_redirect(
        hx_request,
        &url_no_page,
        search_results_count_bar(
            result.shown_results,
            result.total_results,
            Some(card_collection),
        ),
    )
}

async fn handle_initial_search(
    hx_request: bool,
    google_client_id: &str,
    login_url: &str,
    tmdb_service: &TmdbService,
    query: &SearchQuery,
    uri_str: &str,
) -> Response {
    let next_page_url = build_search_url(uri_str, &query.q, query.t, Some(2));
    let result = fetch_search_results(tmdb_service, &query.q, query.t, 1, &next_page_url).await;
    maybe_document(
        hx_request,
        google_client_id,
        login_url,
        search_page(
            query,
            result.shown_results,
            result.total_results,
            &result.cards,
        ),
    )
}

async fn login_get(
    HxRequest(hx_request): HxRequest,
    State(state): State<AppState>,
) -> impl IntoResponse {
    maybe_document(
        hx_request,
        &state.google_client_id,
        state.login_url,
        login_page(),
    )
}

#[derive(Debug, Deserialize)]
struct LoginPostBody {
    credential: String,
    g_csrf_token: String,
}

async fn login_post(
    State(state): State<AppState>,
    cookies: CookieManager,
    session: Session,
    Form(body): Form<LoginPostBody>,
) -> impl IntoResponse {
    let csrf_cookie = cookies.get("g_csrf_token").map(|c| c.value().to_string());
    if let Err(e) = state
        .google_auth_service
        .verify_csrf(csrf_cookie.as_deref(), &body.g_csrf_token)
    {
        tracing::warn!("CSRF verification failed: {}", e);
        return (StatusCode::BAD_REQUEST, "CSRF verification failed").into_response();
    }

    match state
        .google_auth_service
        .verify_token(&body.credential)
        .await
    {
        Ok(user) => {
            tracing::info!("User logged in: {:?}", user);

            let user_id = match get_user_by_google_account_id(&state.db_pool, &user.sub).await {
                Ok(user) => user.id,
                Err(DbError::UserNotFound) => {
                    // TODO: Error handling!
                    create_user(
                        &state.db_pool,
                        Some(&user.sub),
                        user.email.as_deref(),
                        user.name.as_deref(),
                        user.picture.as_deref(),
                    )
                    .await
                    .unwrap()
                    .id
                }
                Err(_) => todo!(),
            };

            if let Err(e) = session.insert("session", AppSession { user_id }).await {
                tracing::error!("Failed to store session: {}", e);
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to create session",
                )
                    .into_response();
            }

            axum::response::Redirect::to("/").into_response()
        }
        Err(e) => {
            tracing::warn!("Token verification failed: {}", e);
            (StatusCode::UNAUTHORIZED, "Invalid credentials").into_response()
        }
    }
}

// TODO: If not logged in, don't logout
async fn logout(session: Session) -> impl IntoResponse {
    if let Err(e) = session.delete().await {
        tracing::error!("Failed to delete session: {}", e);
    }
    axum::response::Redirect::to("/login")
}

async fn profile(
    HxRequest(hx_request): HxRequest,
    State(state): State<AppState>,
    session: AppSession,
) -> impl IntoResponse {
    // TODO: Error handling!
    let user = get_user_by_id(&state.db_pool, session.user_id)
        .await
        .unwrap();
    maybe_document(
        hx_request,
        &state.google_client_id,
        state.login_url,
        profile_page(
            user.email.as_deref(),
            user.username.as_deref(),
            user.picture_url.as_deref(),
            &user.created_at.format(&Rfc2822).unwrap(),
        ),
    )
}

#[derive(Deserialize)]
struct DeleteAccountBody {
    confirm: String,
}

async fn delete_account(
    State(state): State<AppState>,
    app_session: AppSession,
    session: Session,
    Form(body): Form<DeleteAccountBody>,
) -> impl IntoResponse {
    if body.confirm.to_lowercase() != "delete" {
        return (
            StatusCode::NO_CONTENT,
            [("HX-Trigger", "show-account-not-deleted-modal")],
        );
    }

    logout(session).await;
    // TODO: Error handling!
    delete_user(&state.db_pool, app_session.user_id)
        .await
        .unwrap();

    (StatusCode::NO_CONTENT, [("HX-Location", "/")])
}

pub fn index_router() -> Router<AppState> {
    Router::new()
        .route("/", get(index))
        .route("/about", get(about))
        .route("/privacy", get(privacy))
        .route("/search", get(search))
        .route("/login", get(login_get).post(login_post))
        .route("/logout", get(logout))
        .route("/profile", get(profile))
        .route("/deleteAccount", post(delete_account))
}
