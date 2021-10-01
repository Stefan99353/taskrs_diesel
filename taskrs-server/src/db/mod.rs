use crate::CONFIG;

pub mod schema;
pub mod user;
pub mod auth_refresh_token;
pub mod permission;
pub mod category;

pub async fn connect_database() -> Result<sea_orm::DatabaseConnection, sea_orm::DbErr> {
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

    sea_orm::Database::connect(&database_url).await
}