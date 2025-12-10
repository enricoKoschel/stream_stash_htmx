use crate::views::{MainPage, SimpleComponent};
use axum::response::IntoResponse;
use hypertext::prelude::*;

pub async fn index() -> impl IntoResponse {
    maud! {
        MainPage {
            SimpleComponent;
        }
    }
}
