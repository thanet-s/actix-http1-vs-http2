use actix_web::{web, App, HttpResponse, HttpServer};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

async fn hello() -> HttpResponse {
    HttpResponse::Ok().json("Hello from API 2 (HTTP/2)")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("certs/key.pem", SslFiletype::PEM)
        .unwrap();
    builder
        .set_certificate_chain_file("certs/cert.pem")
        .unwrap();

    HttpServer::new(|| App::new().route("/", web::get().to(hello)))
        .bind_openssl("0.0.0.0:8081", builder)?
        .run()
        .await
}
