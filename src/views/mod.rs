use crate::views::layouts::base_layout;
use axum::body::Body;
use axum::http::Response;
use axum::response::IntoResponse;
use axum::response::Redirect;
use maud::Markup;

pub mod components;
pub mod layouts;
pub mod pages;

pub fn maybe_document(hx_request: bool, children: Markup) -> Response<Body> {
    if hx_request {
        children
    } else {
        base_layout(children)
    }
    .into_response()
}

pub fn maybe_redirect(hx_request: bool, redirect_to: &str, children: Markup) -> Response<Body> {
    if hx_request {
        children.into_response()
    } else {
        Redirect::to(redirect_to).into_response()
    }
}
