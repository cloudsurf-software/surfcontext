use axum::{Router, response::Html, routing::get};

use crate::html;

async fn tools_page() -> Html<String> {
    let result = surf_parse::parse(include_str!("../../content/tools.surf"));
    let content = result.doc.to_html();
    Html(html::page(
        "Tools - SurfContext",
        "Tools for creating and managing SurfContext projects: SurfDoc, WaveSite, Surf CLI, and Surf MCP Server.",
        "",
        &content,
    ))
}

pub fn router() -> Router {
    Router::new().route("/tools", get(tools_page))
}
