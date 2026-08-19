use axum::{Router, response::Html, routing::get};

use crate::html;

async fn whats_new_page() -> Html<String> {
    let result = surf_parse::parse(include_str!("../../content/whats_new.surf"));
    let content = result.doc.to_html();
    Html(html::page(
        "What's New in ARDS v4.0 — SurfContext",
        "Nine new capabilities for structuring AI-agent-ready repositories. Living guides, session checkpoints, evidence epistemology, IP safety, context budget management, and more.",
        "",
        &content,
    ))
}

pub fn router() -> Router {
    Router::new().route("/whats-new", get(whats_new_page))
}
