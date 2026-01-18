use crate::views::layouts::base_layout;
use axum::response::{IntoResponse, Redirect, Response};
use maud::Markup;

pub mod components;
pub mod icons;
pub mod layouts;
pub mod pages;

pub fn maybe_document(
    hx_request: bool,
    google_client_id: &str,
    login_redirect_url: &str,
    children: Markup,
) -> Response {
    if hx_request {
        children
    } else {
        base_layout(google_client_id, login_redirect_url, children)
    }
    .into_response()
}

pub fn maybe_redirect(hx_request: bool, redirect_to: &str, children: Markup) -> Response {
    if hx_request {
        children.into_response()
    } else {
        Redirect::to(redirect_to).into_response()
    }
}
