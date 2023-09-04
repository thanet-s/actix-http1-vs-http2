use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Serialize;

#[derive(Serialize)]
struct ResponseObj {
    response: String,
}

async fn hello() -> HttpResponse {
    let data = ResponseObj {
        response: String::from("Hello from API 1 (HTTP/1.1) Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed mollis neque vitae odio ornare, sed hendrerit arcu cursus. Praesent nec tellus id augue semper auctor. Nam ornare diam lacus, sed vulputate ligula ultrices eleifend. Phasellus dolor tellus, imperdiet at ex vel, pellentesque rutrum sapien. Fusce rhoncus, purus vitae accumsan aliquet, orci velit porttitor sapien, eu tincidunt mauris risus vel ligula. Fusce vitae imperdiet ex. Phasellus id ligula at neque mattis euismod. Pellentesque ullamcorper felis interdum elit scelerisque, quis ornare turpis mattis. Integer et risus nec metus ultrices ultricies"),
    };

    HttpResponse::Ok().json(data)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::get().to(hello)))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
