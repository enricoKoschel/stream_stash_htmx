use crate::views::components::MediaCard;
use crate::views::maybe_document;
use crate::views::pages::{about_page, main_page};
use axum::response::IntoResponse;
use axum::{Router, routing::get};
use axum_htmx::HxRequest;
use std::iter;

async fn index(HxRequest(hx_request): HxRequest) -> impl IntoResponse {
    let media: Vec<MediaCard> = iter::repeat_with(|| [
        MediaCard {
            title: "Harry Potter and the Philosopher's Stone",
            year: "2001",
            poster_url: "https://image.tmdb.org/t/p/w600_and_h900_bestv2/wuMc08IPKEatf9rnMNXvIDxqP4W.jpg",
            media_page_url: "/media/movie/1",
        },
        MediaCard {
            title: "Breaking Bad",
            year: "2008",
            poster_url: "https://image.tmdb.org/t/p/w600_and_h900_bestv2/ztkUQFLlC19CCMYHW9o1zWhJRNq.jpg",
            media_page_url: "/media/tv/1",
        },
    ]).flatten().take(50).collect();

    maybe_document(hx_request, main_page(&media))
}

async fn about(HxRequest(hx_request): HxRequest) -> impl IntoResponse {
    maybe_document(hx_request, about_page())
}

pub fn index_router() -> Router {
    Router::<()>::new()
        .route("/", get(index))
        .route("/about", get(about))
}
