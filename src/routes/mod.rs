mod landing;
mod spec;
mod getting_started;
mod tools;
mod whats_new;
mod download;

use axum::Router;

pub fn router() -> Router {
    Router::new()
        .merge(landing::router())
        .merge(spec::router())
        .merge(getting_started::router())
        .merge(tools::router())
        .merge(whats_new::router())
        .merge(download::router())
}
