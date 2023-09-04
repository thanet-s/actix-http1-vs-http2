use actix_web::{web, App, HttpResponse, HttpServer};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use serde::Serialize;

#[derive(Serialize)]
struct ResponseObj {
    response: String,
}

async fn hello() -> HttpResponse {
    let data = ResponseObj {
        response: String::from("Hello from API 2 (HTTP/2) Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed mollis neque vitae odio ornare, sed hendrerit arcu cursus. Praesent nec tellus id augue semper auctor. Nam ornare diam lacus, sed vulputate ligula ultrices eleifend. Phasellus dolor tellus, imperdiet at ex vel, pellentesque rutrum sapien. Fusce rhoncus, purus vitae accumsan aliquet, orci velit porttitor sapien, eu tincidunt mauris risus vel ligula. Fusce vitae imperdiet ex. Phasellus id ligula at neque mattis euismod. Pellentesque ullamcorper felis interdum elit scelerisque, quis ornare turpis mattis. Integer et risus nec metus ultrices ultricies"),
    };

    HttpResponse::Ok().json(data)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("/certs/localhost.key", SslFiletype::PEM)
        .unwrap();
    builder
        .set_certificate_chain_file("/certs/localhost.crt")
        .unwrap();

    HttpServer::new(|| App::new().route("/", web::get().to(hello)))
        .bind_openssl("0.0.0.0:8081", builder)?
        .run()
        .await
}
