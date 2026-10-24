mod landing;
pub mod spec;
mod getting_started;
mod tools;
mod whats_new;
mod download;
mod talks;

use axum::Router;

pub fn router() -> Router {
    Router::new()
        .merge(landing::router())
        .merge(spec::router())
        .merge(getting_started::router())
        .merge(tools::router())
        .merge(whats_new::router())
        .merge(download::router())
        .merge(talks::router())
}

#[cfg(test)]
mod content_parse_tests {
    use surf_parse::Severity;

    fn error_diagnostics(source: &str) -> Vec<String> {
        surf_parse::parse(source)
            .diagnostics
            .iter()
            .filter(|d| matches!(d.severity, Severity::Error))
            .map(|d| format!("{:?}", d))
            .collect()
    }

    /// Every `.surf` document this site serves must parse cleanly with
    /// `surf-parse`. The route tests assert the page *shell*, so a parse
    /// regression would ship a blank or mangled body without failing anything —
    /// this suite is what makes that visible.
    fn assert_parses(label: &str, source: &str) {
        let errors = error_diagnostics(source);
        assert!(
            errors.is_empty(),
            "{label}: surf-parse reported {} error diagnostic(s): {}",
            errors.len(),
            errors.join(" | ")
        );

        let html = surf_parse::parse(source).doc.to_html();
        assert!(!html.trim().is_empty(), "{label}: parsed to empty HTML");
    }

    #[test]
    fn landing_surf_parses() {
        assert_parses("content/landing.surf", include_str!("../../content/landing.surf"));
    }

    #[test]
    fn tools_surf_parses() {
        assert_parses("content/tools.surf", include_str!("../../content/tools.surf"));
    }

    #[test]
    fn whats_new_surf_parses() {
        assert_parses("content/whats_new.surf", include_str!("../../content/whats_new.surf"));
    }

    /// SPEC.surf carries `type: specification` in its frontmatter.
    ///
    /// This used to be the site's one tolerated parse defect: the type was
    /// outside the vocabulary surf-parse accepted, so the parser rejected the
    /// whole frontmatter block and the test pinned that single error by code
    /// and subject. D-SC-3 resolved it in the parser's favour — surf-parse
    /// 0.19.1 adds `specification` to the type vocabulary additively, and the
    /// canon document's declared type is unchanged.
    ///
    /// The pin therefore inverts: SPEC.surf must now yield ZERO error
    /// diagnostics, like every other document this site serves. It is asserted
    /// separately from the corpus above because this document is the one that
    /// carried the defect — a regression here is a regression in the
    /// vocabulary, not in the body.
    #[test]
    fn spec_surf_has_no_frontmatter_defect() {
        let errors = error_diagnostics(super::spec::SPEC_SOURCE);
        assert!(
            errors.is_empty(),
            "SPEC.surf must parse with zero error diagnostics (D-SC-3), got {}: {}",
            errors.len(),
            errors.join(" | ")
        );
    }

    /// `type: specification` was never the ONLY thing keeping SPEC.surf's
    /// frontmatter out of the parse — it was merely the first field serde
    /// tripped on, so the old single-error pin masked what came after it.
    ///
    /// With the type accepted, the next field fails: `version: 5.2` is a
    /// decimal, and `FrontMatter::version` is an integer, so the block is
    /// still discarded — now as a P005 *warning* rather than an error. That is
    /// why the pin above is about errors and this one is about the residual.
    ///
    /// SPEC.surf is a byte-identical mirror of the canon document and is not
    /// this lane's to edit, and widening the parser's `version` field was not
    /// ruled on, so the state is pinned honestly instead of asserted away: the
    /// residual is exactly one warning, about `version`, and nothing else. If
    /// the decimal-version question is later ruled on either way, this test is
    /// what fails and points at the ruling.
    #[test]
    fn spec_surf_residual_is_only_the_decimal_version_warning() {
        let parsed = surf_parse::parse(super::spec::SPEC_SOURCE);
        let residual: Vec<String> = parsed
            .diagnostics
            .iter()
            .map(|d| format!("{:?}", d))
            .collect();
        assert_eq!(
            residual.len(),
            1,
            "SPEC.surf residual diagnostics changed, got: {}",
            residual.join(" | ")
        );
        assert!(
            residual[0].contains("P005") && residual[0].contains("version"),
            "SPEC.surf residual is no longer the known decimal-version warning: {}",
            residual[0]
        );
        // Consequence of that residual, stated rather than hidden: the block is
        // dropped, so the type the document declares does not reach the parse.
        assert!(
            parsed.doc.front_matter.is_none(),
            "SPEC.surf frontmatter now survives — the decimal-version defect was \
             resolved; update this pin and the `version` note in CONTEXT.surf"
        );
    }

    /// The spec is the largest SurfDoc the site renders — tables, fenced code,
    /// and 45 numbered sections. A silent truncation would still "parse", so
    /// assert the rendered body reaches the amendment and both end appendices.
    #[test]
    fn spec_surf_renders_end_to_end() {
        let html = surf_parse::parse(super::spec::SPEC_SOURCE).doc.to_html();
        assert!(!html.trim().is_empty(), "SPEC.surf rendered to empty HTML");
        for marker in [
            "Root Context",
            "SurfDoc-Native Context",
            "Appendix A",
            "Appendix D",
        ] {
            assert!(html.contains(marker), "SPEC.surf render is missing {marker:?}");
        }
    }
}
