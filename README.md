# surfcontext.org

The specification site for **ARDS** — the Agent-Ready Documentation Standard, an open
standard for AI-readable project documentation published by CloudSurf Software LLC.

The site serves the canonical ARDS v4.0 specification, a getting-started guide, a tools
directory, release notes, and a downloadable starter kit.

## Stack

- Rust + [axum](https://github.com/tokio-rs/axum) — a single binary serves every page
- [surf-parse](https://github.com/cloudsurf-software/surf-parse) — renders `.surf` content
  files to HTML at request time
- Static assets under `static/` and `assets/`

## Build and run

```bash
cargo run
```

Open [http://localhost:3000](http://localhost:3000). Set `PORT` to change the port.

Run tests with `cargo test`.

## How content works

Page content lives in `.surf` files under `content/`:

| File | Page |
|------|------|
| `content/landing.surf` | `/` |
| `content/spec.surf` | `/spec` — the canonical ARDS v4.0 specification |
| `content/getting_started.surf` | `/getting-started` |
| `content/tools.surf` | `/tools` |
| `content/whats_new.surf` | `/whats-new` |

Content is compiled into the binary via `include_str!`, so **editing a `.surf` file
requires a rebuild** (`cargo run` again) to see the change.

`POST /api/download` builds a starter-kit ZIP on the fly (`src/routes/download.rs`).

## Deploys

CI (`.github/workflows/deploy.yml`) deploys to Fly.io on push:

| Branch | Fly app | Config |
|--------|---------|--------|
| `main` | `surfcontext` (production, surfcontext.org) | `fly.production.toml` |
| `staging` | `surfcontext-staging` | `fly.staging.toml` |
| `develop` | `surfcontext-develop` | `fly.develop.toml` |

Each deploy runs `cargo test` first (except develop), then a smoke test against the
deployed app.

## Smoke test

```bash
scripts/smoke-test.sh <base-url>
# e.g.
scripts/smoke-test.sh http://localhost:3000
```

Checks the main pages, static assets, 404 handling, and the starter-kit download.

## License

- Website code: MIT — see [LICENSE](LICENSE)
- Specification and site content (`content/`): CC BY-SA 4.0 — see
  [LICENSE-CONTENT](LICENSE-CONTENT)

Published by [CloudSurf Software LLC](https://cloudsurf.com).
