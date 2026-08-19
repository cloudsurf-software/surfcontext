use axum::{Router, response::IntoResponse, routing::post, http::header};
use serde::Deserialize;
use std::io::Write;

#[derive(Deserialize)]
struct DownloadForm {
    project_name: String,
    description: String,
    #[serde(default)]
    stack: String,
}

async fn download_starter_kit(
    axum::Form(form): axum::Form<DownloadForm>,
) -> impl IntoResponse {
    let project = if form.project_name.trim().is_empty() {
        "my-project".to_string()
    } else {
        form.project_name.trim().to_string()
    };

    let desc = if form.description.trim().is_empty() {
        "A project using SurfContext".to_string()
    } else {
        form.description.trim().to_string()
    };

    let stack_items: Vec<&str> = form.stack.split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    let stack_line = if stack_items.is_empty() {
        String::new()
    } else {
        format!("**Stack**: {}\n", stack_items.join(", "))
    };

    let context_md = format!(
        r#"<!-- SurfContext ARDS v4.0 — surfcontext.org -->

# {project}

{desc}

{stack_line}
## Key Files

| Path | Purpose |
|------|---------|
| `.context/docs/` | Deep context documents |
| `.context/agents/` | Specialized agent definitions |
| `.context/guides/` | Living how-to guides |

## Architecture

Describe your project architecture here.

## Stack & Development

Describe your development workflow here.
"#
    );

    let claude_md = format!(
        "<!-- SurfContext ARDS v4.0 — surfcontext.org -->\n\
         <!-- GENERATED — edit CONTEXT.md, then run surf sync -->\n\n\
         # {project}\n\n{desc}\n\n{stack_line}"
    );

    let surfcontext_json = format!(
        r#"{{
  "version": "4.0",
  "project": {{
    "name": "{project}",
    "type": "product"
  }},
  "generation": {{
    "targets": ["CLAUDE.md", "AGENTS.md"],
    "strategy": "compact"
  }}
}}"#
    );

    let agent_md = r#"---
name: code-reviewer
description: "Review code changes for quality, security, and consistency (80-150 chars)"
tools: [Read, Grep, Glob]
---

# Code Reviewer

Review code changes for:
- Security vulnerabilities (OWASP top 10)
- Performance issues
- Code style consistency
- Test coverage gaps
"#;

    let buf = Vec::new();
    let cursor = std::io::Cursor::new(buf);
    let mut zip_writer = zip::ZipWriter::new(cursor);
    let options = zip::write::SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated);

    let _ = zip_writer.start_file("CONTEXT.md", options);
    let _ = zip_writer.write_all(context_md.as_bytes());

    let _ = zip_writer.start_file("CLAUDE.md", options);
    let _ = zip_writer.write_all(claude_md.as_bytes());

    let _ = zip_writer.start_file("AGENTS.md", options);
    let _ = zip_writer.write_all(claude_md.as_bytes());

    let _ = zip_writer.start_file("surfcontext.json", options);
    let _ = zip_writer.write_all(surfcontext_json.as_bytes());

    let _ = zip_writer.start_file(".context/agents/code-reviewer.md", options);
    let _ = zip_writer.write_all(agent_md.as_bytes());

    let _ = zip_writer.start_file(".context/docs/.gitkeep", options);
    let _ = zip_writer.write_all(b"");

    let _ = zip_writer.start_file(".context/guides/.gitkeep", options);
    let _ = zip_writer.write_all(b"");

    let cursor = zip_writer.finish().unwrap();
    let bytes = cursor.into_inner();

    let filename = format!("{}-surfcontext.zip", project.to_lowercase().replace(' ', "-"));

    (
        [
            (header::CONTENT_TYPE, "application/zip".to_string()),
            (header::CONTENT_DISPOSITION, format!("attachment; filename=\"{filename}\"")),
        ],
        bytes,
    )
}

pub fn router() -> Router {
    Router::new().route("/api/download", post(download_starter_kit))
}
