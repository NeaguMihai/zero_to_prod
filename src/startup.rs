use std::net::TcpListener;

use crate::common::configuration::database::postgres_config::PgPool;
use crate::routes::{health_check, subscribe};
use actix_web::{dev::Server, web, App, HttpServer};
use tracing_actix_web::TracingLogger;

pub fn run(listener: TcpListener, connection: PgPool) -> Result<Server, std::io::Error> {
    let atomic_connection = web::Data::new(connection);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/health-check", web::get().to(health_check))
            .route("/subscribe", web::post().to(subscribe))
            .app_data(atomic_connection.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
