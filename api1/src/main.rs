use actix_web::{web, App, HttpResponse, HttpServer};

async fn hello() -> HttpResponse {
    HttpResponse::Ok().json("Hello from API 1 (HTTP/1.1)")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::get().to(hello)))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
