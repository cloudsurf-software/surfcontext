// SurfContext specification website — surfcontext.org
// CloudSurf Software LLC
mod html;
mod routes;

use axum::{Router, routing::get};
use tower_http::services::ServeDir;
use tracing_subscriber::EnvFilter;

async fn surfdoc_css() -> impl axum::response::IntoResponse {
    (
        [(axum::http::header::CONTENT_TYPE, "text/css; charset=utf-8")],
        surf_parse::SURFDOC_CSS,
    )
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let app = Router::new()
        .merge(routes::router())
        .route("/static/css/surf-ui.css", get(surfdoc_css))
        .nest_service("/static", ServeDir::new("static"))
        .nest_service("/assets", ServeDir::new("assets"));

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{port}");
    tracing::info!("surfcontext.org listening on {addr}");

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
