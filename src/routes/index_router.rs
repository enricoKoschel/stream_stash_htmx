use crate::data_source::db::{
    create_user, delete_user, get_media_by_user_id, get_user_by_google_account_id, get_user_by_id,
};
use crate::data_source::search::{SearchQuery, build_search_url, fetch_search_results};
use crate::data_source::tmdb::TmdbService;
use crate::data_source::{MediaType, TmdbMedia};
use crate::views::components::card_collection;
use crate::views::layouts::search_layout::search_results_count_bar;
use crate::views::pages::{
    about_page, login_page, main_page, privacy_page, profile_page, search_page,
};
use crate::views::{maybe_document, maybe_redirect};
use crate::{AppSession, AppState, MaybeAppSession};
use axum::Form;
use axum::Router;
use axum::extract::{Query, State};
use axum::http::{StatusCode, Uri};
use axum::response::{IntoResponse, Redirect, Response};
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
    let media = match get_media_by_user_id(&state.db_pool, session.user_id).await {
        Ok(media) => media,
        Err(e) => {
            tracing::error!("Failed to fetch media for user {}: {}", session.user_id, e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to load media").into_response();
        }
    };

    let mut cards = Vec::with_capacity(media.len());
    for m in &media {
        let media_result = match m.r#type {
            MediaType::Movies => state
                .tmdb_service
                .movie_details(m.id as i32)
                .await
                .map(TmdbMedia::Movie),
            MediaType::TvShows => state
                .tmdb_service
                .tv_show_details(m.id as i32)
                .await
                .map(TmdbMedia::TvShow),
        };

        match media_result {
            Ok(media) => cards.push(state.tmdb_service.map_media_to_card(&media, None)),
            Err(e) => {
                tracing::warn!("Failed to fetch details for media id {}: {}", m.id, e);
                // Continue loading other media even if one fails
                continue;
            }
        }
    }

    maybe_document(
        hx_request,
        None,
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
        Some("About"),
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
        Some("Privacy Policy"),
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
        return StatusCode::NO_CONTENT.into_response();
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
    let result =
        match fetch_search_results(tmdb_service, &query.q, query.t, page, &next_page_url).await {
            Ok(result) => result,
            Err(e) => {
                tracing::error!("Search failed for query '{}': {}", query.q, e);
                return (StatusCode::INTERNAL_SERVER_ERROR, "Search failed").into_response();
            }
        };

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
    let result =
        match fetch_search_results(tmdb_service, &query.q, query.t, 1, &next_page_url).await {
            Ok(result) => result,
            Err(e) => {
                tracing::error!("Search failed for query '{}': {}", query.q, e);
                return (StatusCode::INTERNAL_SERVER_ERROR, "Search failed").into_response();
            }
        };
    maybe_document(
        hx_request,
        Some(&query.q),
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
    session: MaybeAppSession,
) -> impl IntoResponse {
    // Redirect to / if already logged in
    if session.0.is_some() {
        return Redirect::to("/").into_response();
    }

    maybe_document(
        hx_request,
        Some("Login"),
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
                Ok(Some(user)) => user.id,
                Ok(None) => {
                    match create_user(
                        &state.db_pool,
                        Some(&user.sub),
                        user.email.as_deref(),
                        user.name.as_deref(),
                        user.picture.as_deref(),
                    )
                    .await
                    {
                        Ok(new_user) => {
                            tracing::info!("Created new user with id {}", new_user.id);
                            new_user.id
                        }
                        Err(e) => {
                            tracing::error!("Failed to create user: {}", e);
                            return (
                                StatusCode::INTERNAL_SERVER_ERROR,
                                "Failed to create user account",
                            )
                                .into_response();
                        }
                    }
                }
                Err(e) => {
                    tracing::error!("Database error while fetching user: {}", e);
                    return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response();
                }
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

async fn logout(session: Session) -> impl IntoResponse {
    if let Err(e) = session.delete().await {
        tracing::error!("Failed to delete session: {}", e);
    }
    Redirect::to("/login")
}

async fn profile(
    HxRequest(hx_request): HxRequest,
    State(state): State<AppState>,
    session: AppSession,
) -> impl IntoResponse {
    let user = match get_user_by_id(&state.db_pool, session.user_id).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            tracing::error!(
                "Failed to fetch user {}: User does not exist",
                session.user_id
            );
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to load profile").into_response();
        }
        Err(e) => {
            tracing::error!("Failed to fetch user {}: {}", session.user_id, e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to load profile").into_response();
        }
    };

    let created_at_str = match user.created_at.assume_utc().format(&Rfc2822) {
        Ok(formatted) => formatted,
        Err(e) => {
            tracing::warn!("Failed to format date: {}", e);
            "Unknown".to_string()
        }
    };

    maybe_document(
        hx_request,
        Some("Profile"),
        &state.google_client_id,
        state.login_url,
        profile_page(
            user.email.as_deref(),
            user.username.as_deref(),
            user.picture_url.as_deref(),
            &created_at_str,
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
        )
            .into_response();
    }

    if let Err(e) = delete_user(&state.db_pool, app_session.user_id).await {
        tracing::error!("Failed to delete user {}: {}", app_session.user_id, e);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "There was an error while trying to delete your account. Please try again.",
        )
            .into_response();
    }

    logout(session).await;

    (StatusCode::NO_CONTENT, [("HX-Location", "/")]).into_response()
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
