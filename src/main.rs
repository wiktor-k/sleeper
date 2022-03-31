use actix_web::{web, App, HttpServer, Responder};
use std::thread;
use std::time::Duration;

async fn hello_world() -> impl Responder {
    "Hello World!"
}

async fn sleeper() -> impl Responder {
    thread::sleep(Duration::from_secs(50));
    "sleepy time"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(hello_world))
            .route("/sleep", web::get().to(sleeper))
    })
    .bind("0.0.0.0:8080")
    .expect("Can not bind to port 8080")
    .run()
    .await
}
