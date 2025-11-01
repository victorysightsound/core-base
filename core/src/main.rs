use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

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
async fn sync_pull() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "data": []
    }))
}

#[post("/sync/push")]
async fn sync_push(body: web::Json<serde_json::Value>) -> impl Responder {
    println!("Received sync data: {:?}", body);
    HttpResponse::Ok().json(serde_json::json!({
        "status": "received"
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting CORE API server on http://localhost:8080");
    HttpServer::new(|| {
        App::new()
            .service(login)
            .service(sync_pull)
            .service(sync_push)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
