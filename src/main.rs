use std::net::TcpListener;

use zero_to_prod::{common::configuration::{ConfigService, env::Env}, run};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    ConfigService::init();
    let port: u16 = ConfigService::get(Env::ServerPort).parse().unwrap();
    let host: String = ConfigService::get(Env::ServerHost);
    let listener = TcpListener::bind(format!("{}:{}", host, port)).expect("Failed to bind random port");
    run(listener)?.await
}
