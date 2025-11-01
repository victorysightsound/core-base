# Windsurf Global Rules

- CORE is API-first: all business logic lives in /core (Rust).
- The UI (/webui) only calls the API; never touches the DB directly.
- The bridge (/bridge) is minimal, allow-listed, and has no business logic.
- Use SQLite locally, Cloudflare D1 in the cloud (same schema as much as possible).
- Enforce RBAC and tenant isolation at the API layer.
- Keep modules small, well-named, and documented with short READMEs.
