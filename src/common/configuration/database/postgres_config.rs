use diesel::Connection;
use diesel::{r2d2::ConnectionManager, r2d2::Pool, PgConnection};

use crate::common::configuration::database::DatabaseConnectionOptions;
use crate::common::configuration::{env::Env, ConfigService};

use super::{DatabaseConnectionConfig, DbPool};

pub type PgPool = DbPool<PgConnection>;

pub struct PgConnectionFactory {}

impl DatabaseConnectionConfig<PgConnection> for PgConnectionFactory {
    fn get_connection_string(options: DatabaseConnectionOptions) -> String {
        let host = options
            .host
            .unwrap_or_else(|| ConfigService::get(Env::DbHost));
        let port = options
            .port
            .unwrap_or_else(|| ConfigService::get(Env::DbPort));
        let user = options
            .user
            .unwrap_or_else(|| ConfigService::get(Env::DbUser));
        let password = options
            .password
            .unwrap_or_else(|| ConfigService::get(Env::DbPassword));
        let database = options
            .database
            .unwrap_or_else(|| ConfigService::get(Env::DbName));
        let connection_string = format!(
            "postgres://{}:{}@{}:{}/{}",
            user, password, host, port, database
        );
        connection_string
    }

    fn get_connection_pool(
        options: DatabaseConnectionOptions,
    ) -> Result<DbPool<PgConnection>, r2d2::Error> {
        let connection_string = PgConnectionFactory::get_connection_string(options);
        let manager = ConnectionManager::<PgConnection>::new(connection_string);
        let pool = Pool::builder().build(manager);
        match pool {
            Ok(c) => {
                println!("Connected to database");
                Ok(c)
            }
            Err(e) => Err(e),
        }
    }
    fn get_connection(options: DatabaseConnectionOptions) -> PgConnection {
        let connection_string = PgConnectionFactory::get_connection_string(options);
        PgConnection::establish(connection_string.as_str())
            .unwrap_or_else(|_| panic!("Error connecting to {}", connection_string))
    }
}
