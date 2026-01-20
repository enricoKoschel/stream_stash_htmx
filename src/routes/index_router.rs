use crate::data_source::db::{DbError, create_user, get_user_by_google_account_id};
use crate::data_source::search::{SearchQuery, build_search_url, fetch_search_results};
use crate::data_source::tmdb::TmdbService;
use crate::views::components::{card_collection, media_card, search_results_count_bar};
use crate::views::pages::{about_page, login_page, main_page, privacy_page, search_page};
use crate::views::{maybe_document, maybe_redirect};
use crate::{AppSession, AppState};
use axum::Form;
use axum::extract::{Path, Query, State};
use axum::http::{StatusCode, Uri};
use axum::response::{IntoResponse, Response};
use axum::{Router, routing::get};
use axum_cookie::CookieManager;
use axum_htmx::HxRequest;
use maud::Markup;
use serde::Deserialize;
use std::iter;
use tower_sessions::Session;

async fn index(
    HxRequest(hx_request): HxRequest,
    State(state): State<AppState>,
    path: Option<Path<usize>>,
) -> impl IntoResponse {
    let count = path.map_or(50, |p| p.0);
    let media = generate_sample_media(count);
    maybe_document(
        hx_request,
        &state.google_client_id,
        state.login_url,
        main_page(&media),
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

fn generate_sample_media(count: usize) -> Vec<Markup> {
    iter::repeat_with(|| [
        media_card(
            "Harry Potter and the Philosopher's Stone",
            "2001",
            Some("https://image.tmdb.org/t/p/w600_and_h900_bestv2/wuMc08IPKEatf9rnMNXvIDxqP4W.jpg"),
            "",
            None,
        ),
        media_card(
            "Breaking Bad",
            "2008",
            Some("https://image.tmdb.org/t/p/w600_and_h900_bestv2/ztkUQFLlC19CCMYHW9o1zWhJRNq.jpg"),
            "",
            None,
        ),
    ])
    .flatten()
    .take(count)
    .collect()
}

async fn db_test(State(state): State<AppState>, session: AppSession) -> impl IntoResponse {
    format!("{:?}", session)
}

// TODO: Allow account deletion
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

            let account_id = match get_user_by_google_account_id(&state.db_pool, &user.sub).await {
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

            if let Err(e) = session.insert("session", AppSession { account_id }).await {
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
    axum::response::Redirect::to("/login")
}

pub fn index_router() -> Router<AppState> {
    Router::new()
        .route("/", get(index))
        .route("/{count}", get(index))
        .route("/about", get(about))
        .route("/privacy", get(privacy))
        .route("/search", get(search))
        .route("/test", get(db_test))
        .route("/login", get(login_get).post(login_post))
        .route("/logout", get(logout))
}
