use std::net::TcpListener;

use crate::common::configuration::database::postgres_config::PgPool;
use crate::common::configuration::open_api::ApiDoc;
use crate::routes::{health_check, subscribe};
use actix_web::{dev::Server, web, App, HttpServer};
use tracing_actix_web::TracingLogger;
use utoipa::OpenApi;
use utoipa_swagger_ui::{Config, SwaggerUi};

pub fn run(listener: TcpListener, connection: PgPool) -> Result<Server, std::io::Error> {
    let atomic_connection = web::Data::new(connection);
    let openapi = ApiDoc::openapi();
    // let swagger = Config::new(["api-doc/openapi.json"]).;
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .service(health_check)
            .service(subscribe)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-doc/openapi.json", openapi.clone()),
            )
            //register a route to serve a static .html file using actix_files
            .service(actix_files::Files::new(
                "/swagger-ui/{_:.*}",
                "./common/templates/swagger-ui",
            ))
            .app_data(atomic_connection.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
