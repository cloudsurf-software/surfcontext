use axum::{Router, response::Html, routing::get};

use crate::html;

async fn getting_started_page() -> Html<String> {
    let result = surf_parse::parse(include_str!("../../content/getting_started.surf"));
    let content = result.doc.to_html();
    Html(html::page(
        "Getting Started — SurfContext",
        "Set up SurfContext in your project in under 5 minutes. No dependencies required.",
        "",
        &content,
    ))
}

pub fn router() -> Router {
    Router::new().route("/getting-started", get(getting_started_page))
}
