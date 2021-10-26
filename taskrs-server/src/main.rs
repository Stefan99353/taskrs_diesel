#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

use std::collections::HashMap;
use std::sync::RwLock;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;

use taskrs_db::{DbConnection, DbPool};

mod api;
mod config;
mod middleware;
mod models;
pub mod permissions;
pub mod utils;

lazy_static! {
    static ref CONFIG: crate::config::Config =
        crate::config::Config::new().expect("Error reading config");
    static ref PERMISSION_CACHE: RwLock<HashMap<i32, Vec<String>>> = RwLock::new(HashMap::new());
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Logger, .env, Database, Migrations
    log4rs::init_file("config/log.yml", Default::default()).unwrap();
    dotenv().ok();
    let pool = taskrs_db::connect_database(
        &CONFIG.database.host,
        &CONFIG.database.port,
        &CONFIG.database.database,
        &CONFIG.database.user,
        &CONFIG.database.password,
    )
    .unwrap();
    let conn = pool.get().expect("Couldn't get db connection from pool");
    setup_database(&conn);

    start(pool).await
}

async fn start(pool: DbPool) -> std::io::Result<()> {
    HttpServer::new(move || {
        let mut app = App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header(),
            )
            .wrap(actix_web::middleware::Logger::new(
                "%r responded %s in %D ms",
            ))
            .data(pool.clone());

        let mut api_scope = web::scope("/api/v1/");

        // Services
        api_scope = api::users::register(api_scope);
        api_scope = api::auth::register(api_scope);
        api_scope = api::permissions::register(api_scope);
        api_scope = api::categories::register(api_scope);
        api_scope = api::projects::register(api_scope);

        app = app.service(api_scope);
        app
    })
    .bind(format!(
        "{}:{}",
        &CONFIG.server.address, &CONFIG.server.port
    ))?
    .run()
    .await
}

fn setup_database(conn: &DbConnection) {
    taskrs_db::run_migrations(conn).expect("Error running migrations");

    taskrs_db::seed_root_permissions(
        &CONFIG.root_user_email,
        &CONFIG.root_user_password,
        CONFIG.seed_root_permissions,
        conn,
    )
    .expect("Error seeding root user/permissions");

    taskrs_db::update_permissions(permissions::all_permissions(), conn)
        .expect("Error updating permissions");
}
