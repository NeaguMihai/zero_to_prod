use std::net::TcpListener;

use zero_to_prod::{
    common::configuration::{database::DatabaseConnectionFactory, env::Env, ConfigService},
    run,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    ConfigService::init();
    let port: u16 = ConfigService::get(Env::ServerPort).parse().unwrap();
    let host: String = ConfigService::get(Env::ServerHost);
    let listener =
        TcpListener::bind(format!("{}:{}", host, port)).expect("Failed to bind random port");
    let connection_pool = DatabaseConnectionFactory::get_pg_connection_pool()
        .unwrap_or_else(|e| panic!("Failed to connect to database. {}", e));
    run(listener, connection_pool)?.await
}
