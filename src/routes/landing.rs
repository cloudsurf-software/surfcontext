use axum::{Router, response::Html, routing::get};

use crate::html;

async fn landing_page() -> Html<String> {
    let result = surf_parse::parse(include_str!("../../content/landing.surf"));
    let content = result.doc.to_html();
    Html(html::page(
        "SurfContext — One Spec. Every AI Coding Tool.",
        "One format. Every AI coding agent. Define your project context once — all major AI coding agents understand it automatically.",
        "",
        &content,
    ))
}

pub fn router() -> Router {
    Router::new().route("/", get(landing_page))
}
