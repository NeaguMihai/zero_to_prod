use self::postgres_config::{PgConnectionFactory, PgPool};
use diesel::r2d2::ConnectionManager;
pub mod postgres_config;

type DbPool<T> = r2d2::Pool<ConnectionManager<T>>;

pub trait DatabaseConnectionConfig<T> {
    fn get_connection_string() -> String;
    fn get_connection() -> Result<T, r2d2::Error>;
}
pub struct DatabaseConnectionFactory {}

impl DatabaseConnectionFactory {
    pub fn get_pg_connection_pool() -> Result<PgPool, r2d2::Error> {
        PgConnectionFactory::get_connection()
    }
}
