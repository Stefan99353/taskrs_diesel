use diesel::PgConnection;
use diesel::r2d2::ConnectionManager;
use r2d2::Pool;

use crate::CONFIG;

pub mod schema;
pub mod user;
pub mod auth_refresh_token;
pub mod permission;
pub mod category;
pub mod project;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub fn connect_database() -> Result<DbPool, r2d2::Error> {
    debug!("Connecting to database");
    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        &CONFIG.database.user,
        &CONFIG.database.password,
        &CONFIG.database.host,
        &CONFIG.database.port,
        &CONFIG.database.database,
    );
    trace!("Database URL: {}", &database_url);

    let connection_manager = ConnectionManager::<PgConnection>::new(&database_url);
    Pool::builder()
        .build(connection_manager)
        .map_err(|e| {
            error!("Could not create database connections: {}", &e);
            e
        })
}