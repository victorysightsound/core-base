/init-core

Goal:
Scaffold and run the first Rust API inside /core.

Steps:

1) **Add dependencies** to /core/Cargo.toml:
   ```toml
   [dependencies]
   actix-web = "4"
   serde = { version = "1", features = ["derive"] }
   serde_json = "1"
   sqlx = { version = "0.7", features = ["runtime-tokio", "macros", "sqlite"] }
   tokio = { version = "1", features = ["full"] }
