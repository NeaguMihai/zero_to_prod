use crate::common::configuration::database::postgres_config::PgPool;
use crate::common::configuration::open_api::initialize_openapi;
use crate::routes::{health_check, subscribe};
use actix_web::{dev::Server, web, App, HttpServer};
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;
use utoipa_swagger_ui::SwaggerUi;

pub fn run(listener: TcpListener, connection: PgPool) -> Result<Server, std::io::Error> {
    let atomic_connection = web::Data::new(connection);
    let openapi = initialize_openapi();
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .service(health_check)
            .service(subscribe)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-doc/openapi.json", openapi.clone()),
            )
            .app_data(atomic_connection.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
