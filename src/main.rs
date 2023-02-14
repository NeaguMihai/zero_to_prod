use std::net::TcpListener;

use zero_to_prod::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8080").expect("Failed to bind random port");
    run(listener)?.await
}
