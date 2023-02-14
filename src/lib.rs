use std::net::TcpListener;

use actix_web::{dev::Server, web, App, HttpResponse, HttpServer};
use serde::Deserialize;

#[derive(Deserialize)]
struct SubscribeBody {
    name: String,
    email: String,
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn subscribe(_body: web::Json<SubscribeBody>) -> HttpResponse {
    println!("name: {:?}", _body.name);
    println!("body: {:?}", _body.email);
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscribe", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
