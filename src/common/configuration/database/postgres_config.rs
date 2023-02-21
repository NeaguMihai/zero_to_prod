use diesel::{r2d2::ConnectionManager, r2d2::Pool, PgConnection};

use crate::common::configuration::{env::Env, ConfigService};

use super::{DatabaseConnectionConfig, DbPool};

pub type PgPool = DbPool<PgConnection>;

pub struct PgConnectionFactory {}

impl DatabaseConnectionConfig<PgPool> for PgConnectionFactory {
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

    fn get_connection() -> Result<DbPool<PgConnection>, r2d2::Error> {
        let connection_string = PgConnectionFactory::get_connection_string();
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
}
