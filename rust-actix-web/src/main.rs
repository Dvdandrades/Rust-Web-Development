use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct Message {
    message: String,
}

async fn greet() -> impl Responder {
    HttpResponse::Ok().json(Message {
        message: "Hello from Rust API!".to_string(),
    })
}

// Main function to run the HTTP server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().route("/", web::get().to(greet)) // Route "/" to the greet function
    })
    .bind("127.0.0.1:8080")? // Bind the server to localhost:8080
    .run()
    .await
}