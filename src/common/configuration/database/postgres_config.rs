use diesel::{Connection, ConnectionError, PgConnection};

use crate::common::configuration::{env::Env, ConfigService};

use super::DatabaseConnectionConfig;

pub struct PgConnectionFactory {}

impl DatabaseConnectionConfig for PgConnectionFactory {
    type ConnectionType = PgConnection;
    fn get_connection_string() -> String {
        let host = ConfigService::get(Env::DbHost);
        let port = ConfigService::get(Env::DbPort);
        let user = ConfigService::get(Env::DbUser);
        let password = ConfigService::get(Env::DbPassword);
        let database = ConfigService::get(Env::DbName);
        let connection_string = format!(
            "postgres://{}:{}@{}:{}/{}",
            user, password, host, port, database
        );
        connection_string
    }

    fn get_connection() -> Result<PgConnection, ConnectionError> {
        let connection_string = PgConnectionFactory::get_connection_string();
        let connection = PgConnection::establish(&connection_string);
        match connection {
            Ok(c) => {
                println!("Connected to database");
                return Ok(c);
            }
            Err(e) => Err(e),
        }
    }
}
