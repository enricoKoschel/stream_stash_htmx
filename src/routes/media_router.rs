use crate::views::pages::media_page;
use crate::{AppState, views::maybe_document};
use axum::body::Body;
use axum::http::Response;
use axum::{Router, extract::Path, routing::get};
use axum_htmx::HxRequest;

async fn media(
    HxRequest(hx_request): HxRequest,
    Path((media_type, id)): Path<(String, String)>,
) -> Response<Body> {
    // TODO: Get db entry for media, get tmdb entry for media, then display
    let title = "Test title";
    let overview = "Test overview: oisdfhjoisehrfoienvosnhoeruhseohodghbodrshtgosnghoerhtowh4";
    let release_date = "2026-01-08";
    let poster_url =
        Some("https://image.tmdb.org/t/p/w600_and_h900_bestv2//wuMc08IPKEatf9rnMNXvIDxqP4W.jpg");
    let backdrop_url =
        Some("https://image.tmdb.org/t/p/w1920_and_h1080_bestv2//hziiv14OpD73u9gAak4XDDfBKa2.jpg");

    maybe_document(
        hx_request,
        media_page(title, overview, release_date, poster_url, backdrop_url),
    )
}

pub fn media_router() -> Router<AppState> {
    Router::new().route("/{media_type}/{id}", get(media))
}
