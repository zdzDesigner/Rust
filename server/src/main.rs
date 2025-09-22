use actix_web::{App, HttpServer, Responder, get, web};
use std::io;

// #[actix_web::main] // or #[tokio::main]
#[tokio::main] // or #[tokio::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| App::new().service(greet))
        .bind(("127.0.0.1", 8900))?
        .run()
        .await
}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}
