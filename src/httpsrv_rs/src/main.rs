use actix_web::{get, web, App, HttpServer, Responder};

#[get("/")]
async fn hello_world() -> impl Responder {
    "Hello World!"
}

#[get("/bug")]
async fn hello_bug() -> actix_web::Result<String> {
    Err(actix_web::error::ErrorNotImplemented("Intentional error"))
}

#[get("/{name}")]
async fn hello(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", &name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
        .service(hello_world)
        .service(hello_bug)
        .service(hello))
        .bind(("127.0.0.1", 3000))?
        .run()
        .await
}
