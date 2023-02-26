//! src/lib.rs
pub mod common;
pub mod models;
pub mod routes;
pub mod schema;
pub mod startup;

use std::net::TcpListener;

use actix_web::{dev::Server, web, App, HttpServer};
use common::configuration::database::postgres_config::PgPool;
use routes::{health_check, subscribe};

pub fn run(listener: TcpListener, connection: PgPool) -> Result<Server, std::io::Error> {
    let atomic_connection = web::Data::new(connection);
    let server = HttpServer::new(move || {
        App::new()
            .route("/health-check", web::get().to(health_check))
            .route("/subscribe", web::post().to(subscribe))
            .app_data(atomic_connection.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
