use crate::views::layouts::BaseLayout;
use axum::body::Body;
use axum::http::Response;
use axum::response::IntoResponse;
use axum::response::Redirect;
use hypertext::Renderable;
use hypertext::prelude::*;

pub mod components;
pub mod layouts;
pub mod pages;

pub fn maybe_document<R: Renderable>(hx_request: bool, children: R) -> Response<Body> {
    maud! {
        @if hx_request {
            (children)
        } @else {
            BaseLayout {
                (children)
            }
        }
    }
    .into_response()
}

pub fn maybe_redirect<R: Renderable>(hx_request: bool, children: R) -> Response<Body> {
    if hx_request {
        maud! {
            (children)
        }
        .into_response()
    } else {
        Redirect::to("/").into_response()
    }
}
