use actix_cors::Cors;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqlitePoolOptions, Pool, Row, Sqlite};

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct LoginResponse {
    token: String,
}

#[post("/auth/login")]
async fn login(req: web::Json<LoginRequest>) -> impl Responder {
    let response = LoginResponse {
        token: format!("token-for-{}", req.username),
    };
    HttpResponse::Ok().json(response)
}

#[get("/sync/pull")]
async fn sync_pull(db: web::Data<Pool<Sqlite>>) -> impl Responder {
    let rows = sqlx::query("SELECT content FROM notes")
        .fetch_all(db.get_ref())
        .await;

    match rows {
        Ok(notes) => {
            let contents: Vec<String> = notes
                .into_iter()
                .filter_map(|row| row.try_get::<String, _>("content").ok())
                .collect();
            HttpResponse::Ok().json(serde_json::json!({
                "status": "ok",
                "data": contents
            }))
        }
        Err(e) => {
            eprintln!("Database error: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "status": "error"
            }))
        }
    }
}

#[derive(Deserialize)]
struct NewNote {
    content: String,
}

#[post("/sync/push")]
async fn sync_push(
    db: web::Data<Pool<Sqlite>>,
    body: web::Json<NewNote>,
) -> impl Responder {
    let result = sqlx::query("INSERT INTO notes (content) VALUES (?)")
        .bind(&body.content)
        .execute(db.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "status": "saved"
        })),
        Err(e) => {
            eprintln!("Database error: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "status": "error"
            }))
        }
    }
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy"
    }))
}

#[get("/version")]
async fn version() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "app": "CORE API",
        "version": "0.0.1"
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite://core.db")
        .await
        .expect("Failed to connect to database");

    println!("Starting CORE API server on http://localhost:8080");

    HttpServer::new(move || {
        // ✅ Enable CORS so the frontend on port 3000 can access the API
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(db.clone()))
            .service(login)
            .service(sync_pull)
            .service(sync_push)
            .service(health)
            .service(version)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
