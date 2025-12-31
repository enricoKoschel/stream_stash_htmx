use axum::{Router, extract::Path, response::IntoResponse, routing::get};

async fn test(Path((media_type, id)): Path<(String, String)>) -> impl IntoResponse {
    // get db entry for media, get tmdb entry for media, then display
    format!("{media_type}:{id}")
}

pub fn media_router() -> Router {
    Router::<()>::new().route("/{media_type}/{id}", get(test))
}
