use crate::common::{configuration::database::postgres_config::PgPool, core::traits::Module};
use crate::common::configuration::logger::get_trace_layer;
use crate::common::configuration::open_api::initialize_openapi;
use crate::modules::app::AppModule;
use axum::{Extension, Router};
use std::net::TcpListener;
use utoipa_swagger_ui::SwaggerUi;

pub async fn run(listener: TcpListener, connection: PgPool) -> Result<(), std::io::Error> {
    let atomic_connection = Extension(connection);

    let openapi = initialize_openapi();

    let modules: Vec<Box<dyn Module>> = vec![Box::new(AppModule {})];

    let app = Router::new()
        .layer(get_trace_layer())
        .layer(atomic_connection)
        .merge(SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", openapi));

    let app = modules
        .iter()
        .fold(app, |router, module| module.register_controllers(router));

    let _server = axum::Server::from_tcp(listener)
        .expect("Faield to create server from listener")
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}
