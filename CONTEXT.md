# surfcontext.org

This repository is the specification site for ARDS, the Agent-Ready Documentation
Standard — an open standard by CloudSurf Software LLC for AI-readable project
documentation. It is a Rust/axum binary that renders `.surf` content files with
surf-parse and serves them at surfcontext.org. This repo dogfoods its own standard:
`surfcontext.json` declares ARDS version 4.0 and this file is the root context.

## Key Files

| Path | Purpose |
|------|---------|
| `content/spec.surf` | The canonical ARDS v4.0 specification (rendered at `/spec`) |
| `content/landing.surf` | Landing page content (`/`) |
| `content/getting_started.surf` | Getting-started guide (`/getting-started`) |
| `content/tools.surf` | Tools directory (`/tools`) |
| `content/whats_new.surf` | Release notes (`/whats-new`) |
| `src/routes/` | One module per page, plus the starter-kit download endpoint |
| `src/html.rs` | Shared HTML page shell (nav, footer, head) |
| `scripts/smoke-test.sh` | Smoke test run against a deployed base URL |
| `fly.production.toml` / `fly.staging.toml` / `fly.develop.toml` | Fly.io app configs per environment |
| `.github/workflows/deploy.yml` | CI: test → deploy per branch → smoke test |
| `surfcontext.json` | ARDS manifest for this repo (version 4.0) |
| `LICENSE` / `LICENSE-CONTENT` | MIT (code) / CC BY-SA 4.0 (spec and content) |

## Architecture

Request → axum router (`src/main.rs` → `src/routes/`) → surf-parse renders the
compiled-in `.surf` content → `html::page` wraps it in the shared page shell.

- Content files are embedded at build time via `include_str!` — a content edit
  requires a rebuild to take effect.
- `POST /api/download` (`src/routes/download.rs`) assembles a starter-kit ZIP
  in memory and returns it.
- `/static` and `/assets` are served from disk; surf-parse also serves its own
  stylesheet at `/static/css/surf-ui.css`.
- No database, no authentication, no external services at runtime.

## Stack & Development

**Stack**: Rust (edition 2024), axum 0.8, tokio, tower-http, zip,
surf-parse (git dependency, tag v0.9.1)

- Run locally: `cargo run`, then open http://localhost:3000 (set `PORT` to override)
- Test: `cargo test`
- Smoke test a running instance: `scripts/smoke-test.sh <base-url>`
- Editing `content/*.surf` requires a rebuild (content is compiled in)

**Deploys**: push to `main` → CI → Fly.io production app `surfcontext`
(surfcontext.org); `staging` and `develop` branches deploy to their own Fly apps.
Port 3000, scale-to-zero.

## Active Work

- ARDS v4.0 published 2026-08-18; the spec page and starter kit reflect v4.0.
- Next: keep `content/spec.surf` in sync with the canonical standard as it evolves,
  and keep `content/whats_new.surf` current with each release.
