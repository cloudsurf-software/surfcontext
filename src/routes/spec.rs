use axum::{Router, response::Html, routing::get};

use crate::html;

async fn spec_page() -> Html<String> {
    let result = surf_parse::parse(include_str!("../../content/spec.surf"));
    let content = result.doc.to_html();
    Html(html::page(
        "Specification — SurfContext ARDS v4.0",
        "The complete ARDS v4.0 specification for agent-ready project documentation",
        r#"<link rel="stylesheet" href="/static/css/spec.css">"#,
        &content,
    ))
}

pub fn router() -> Router {
    Router::new().route("/spec", get(spec_page))
}
