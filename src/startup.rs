use crate::{common::configuration::database::postgres_config::PgPool, routes::health_check};
use crate::common::configuration::logger::get_trace_layer;
use crate::common::configuration::open_api::initialize_openapi;
use axum::{Extension, Router};
use utoipa_swagger_ui::SwaggerUi;
use std::net::TcpListener;

pub async fn run(listener: TcpListener, connection: PgPool) -> Result<(), std::io::Error> {
    let atomic_connection = Extension(connection);

    let openapi = initialize_openapi();

    let app = Router::new()
        .layer(get_trace_layer())
        .layer(atomic_connection)
        .merge(SwaggerUi::new("/swagger-ui")
         .url("/api-doc/openapi.json", openapi))
        .route("/health-check", axum::routing::get(health_check));

    let _server = axum::Server::from_tcp(listener)
        .expect("Faield to create server from listener")
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}
