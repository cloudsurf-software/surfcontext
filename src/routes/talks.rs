//! Talks: slide decks about ARDS, written as `.surf` files and rendered
//! full-screen by surf-parse's slide renderer (`to_slides_html`).
//!
//! `GET /talks` lists the decks inside the normal page shell. `GET
//! /talks/{slug}` serves one deck as surf-parse's self-contained HTML (its own
//! document, styles and keyboard/touch navigation), without the site shell.

use axum::{
    Router,
    extract::Path,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
};

use crate::html;

/// One talk. Decks are compiled in with `include_str!`, like every other
/// content file, so adding or editing one needs a rebuild.
pub struct Talk {
    /// URL segment: `/talks/{slug}`. Lowercase letters, digits and `-` only.
    pub slug: &'static str,
    pub title: &'static str,
    pub summary: &'static str,
    pub source: &'static str,
}

/// Every talk the site serves, in listing order.
pub const TALKS: &[Talk] = &[Talk {
    slug: "ards-in-ten-minutes",
    title: "ARDS in Ten Minutes",
    summary: "The problem, the three files, and how every agent gets its own config from one source.",
    source: include_str!("../../content/talks/ards-in-ten-minutes.surf"),
}];

/// The talk with this slug, if there is one.
pub fn find(slug: &str) -> Option<&'static Talk> {
    TALKS.iter().find(|t| t.slug == slug)
}

/// A talk rendered as a complete, self-contained deck document.
pub fn deck_html(talk: &Talk) -> String {
    surf_parse::parse(talk.source).doc.to_slides_html()
}

async fn talks_index() -> Html<String> {
    let items: String = TALKS
        .iter()
        .map(|t| {
            format!(
                r#"<li><a href="/talks/{slug}">{title}</a>: {summary}</li>"#,
                slug = t.slug,
                title = t.title,
                summary = t.summary,
            )
        })
        .collect();
    let content = format!(
        r#"<main class="surfdoc"><h1>Talks</h1>
<p>Slide decks about ARDS, written in SurfDoc and rendered with surf-parse. In a deck, press the right arrow or Space to advance, F for full screen and N for speaker notes.</p>
<ul>{items}</ul></main>"#
    );
    Html(html::page(
        "Talks — SurfContext",
        "Slide decks about the Agent-Ready Documentation Standard, written in SurfDoc.",
        "",
        &content,
    ))
}

async fn talk_deck(Path(slug): Path<String>) -> Response {
    match find(&slug) {
        Some(talk) => Html(deck_html(talk)).into_response(),
        None => (StatusCode::NOT_FOUND, "talk not found").into_response(),
    }
}

pub fn router() -> Router {
    Router::new()
        .route("/talks", get(talks_index))
        .route("/talks/{slug}", get(talk_deck))
}

#[cfg(test)]
mod tests {
    use super::*;
    use surf_parse::Severity;

    #[test]
    fn every_talk_parses_without_errors() {
        for talk in TALKS {
            let errors: Vec<String> = surf_parse::parse(talk.source)
                .diagnostics
                .iter()
                .filter(|d| matches!(d.severity, Severity::Error))
                .map(|d| format!("{d:?}"))
                .collect();
            assert!(errors.is_empty(), "{}: {}", talk.slug, errors.join(" | "));
        }
    }

    #[test]
    fn every_talk_renders_as_a_deck_with_all_its_slides() {
        for talk in TALKS {
            let html = deck_html(talk);
            assert!(html.starts_with("<!DOCTYPE html>"), "{}: standalone document", talk.slug);
            let expected = talk.source.lines().filter(|l| l.starts_with("::slide")).count();
            assert!(expected > 0, "{}: has explicit slides", talk.slug);
            assert!(
                html.contains(&format!("data-slides=\"{expected}\"")),
                "{}: expected {expected} slides in the deck",
                talk.slug
            );
            assert!(html.contains(talk.title), "{}: title rendered", talk.slug);
        }
    }

    #[test]
    fn slugs_are_unique_and_url_safe() {
        for (i, talk) in TALKS.iter().enumerate() {
            assert!(
                !talk.slug.is_empty()
                    && talk
                        .slug
                        .chars()
                        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-'),
                "{}: slug must be lowercase letters, digits and -",
                talk.slug
            );
            assert!(
                TALKS[i + 1..].iter().all(|other| other.slug != talk.slug),
                "{}: duplicate slug",
                talk.slug
            );
        }
    }

    #[test]
    fn unknown_slug_is_not_found() {
        assert!(find("no-such-talk").is_none());
        assert!(find("ards-in-ten-minutes").is_some());
    }
}
