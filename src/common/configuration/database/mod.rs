use diesel::{ConnectionError, PgConnection};

use self::postgres_config::PgConnectionFactory;

pub mod postgres_config;

pub trait DatabaseConnectionConfig {
    type ConnectionType;
    fn get_connection_string() -> String;
    fn get_connection() -> Result<Self::ConnectionType, ConnectionError>;
}

pub struct DatabaseConnectionFactory {}

impl DatabaseConnectionFactory {
    pub fn get_pg_connection() -> Result<PgConnection, ConnectionError> {
        PgConnectionFactory::get_connection()
    }
}
