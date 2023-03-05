use self::postgres_config::{PgConnectionFactory, PgPool};
use diesel::{
    r2d2::{ConnectionManager, R2D2Connection},
    PgConnection,
};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub mod postgres_config;

type DbPool<T> = r2d2::Pool<ConnectionManager<T>>;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("../migrations");

pub fn run_migrations<DB>(connection: &mut impl MigrationHarness<DB>)
where
    DB: diesel::backend::Backend + 'static,
{
    match connection.run_pending_migrations(MIGRATIONS) {
        Ok(r) => {
            if r.is_empty() {
                println!("No migrations to run");
            } else {
                println!("Migrations ran successfully");
                r.iter().for_each(|m| println!("Ran migration: {}", m));
            }
        }
        Err(e) => panic!("Error running migrations: {}", e),
    }
}

#[derive(Debug, Default)]
pub struct DatabaseConnectionOptions {
    pub host: Option<String>,
    pub port: Option<String>,
    pub user: Option<String>,
    pub password: Option<String>,
    pub database: Option<String>,
}

// impl Default for DatabaseConnectionOptions {
//     fn default() -> Self {
//         DatabaseConnectionOptions {
//             host: None,
//             port: None,
//             user: None,
//             password: None,
//             database: None,
//         }
//     }
// }

pub trait DatabaseConnectionConfig<T>
where
    T: R2D2Connection + 'static,
{
    fn get_connection_string(options: DatabaseConnectionOptions) -> String;
    fn get_connection_pool(options: DatabaseConnectionOptions) -> Result<DbPool<T>, r2d2::Error>;
    fn get_connection(options: DatabaseConnectionOptions) -> T;
}
pub struct DatabaseConnectionFactory {}

impl DatabaseConnectionFactory {
    pub fn get_pg_connection_string(options: DatabaseConnectionOptions) -> String {
        PgConnectionFactory::get_connection_string(options)
    }
    pub fn get_pg_connection(options: DatabaseConnectionOptions) -> PgConnection {
        PgConnectionFactory::get_connection(options)
    }
    pub fn get_pg_connection_pool(
        options: DatabaseConnectionOptions,
    ) -> Result<PgPool, r2d2::Error> {
        PgConnectionFactory::get_connection_pool(options)
    }
}
