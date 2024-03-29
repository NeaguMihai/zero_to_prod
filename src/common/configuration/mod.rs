//! src/configuration.rs

use self::env::Env;

pub mod database;
pub mod env;
pub mod logger;
pub mod open_api;

pub struct ConfigService {}

impl ConfigService {
    pub fn init() {
        dotenvy::dotenv().expect("Failed to load .env file");
    }
    pub fn get(key: Env) -> String {
        let val = dotenvy::var::<String>(key.to_string());
        match val {
            Ok(v) => v,
            Err(e) => {
                println!("Error: {}", e);
                "".to_string()
            }
        }
    }
}
