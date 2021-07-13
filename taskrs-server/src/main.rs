#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use dotenv::dotenv;

use crate::db::DbPool;
use diesel::PgConnection;
use crate::db::user::User;

mod config;
pub mod db;
mod api;
mod middleware;
mod models;

embed_migrations!("migrations");
lazy_static! {
    static ref CONFIG: crate::config::Config = crate::config::Config::new().expect("Error reading config");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Logger, .env, Database, Migrations
    log4rs::init_file("config/log.yml", Default::default()).unwrap();
    dotenv().ok();
    let pool = db::connect_database().unwrap();
    let conn = pool.get().expect("Couldn't get db connection from pool");
    run_migrations(&conn).expect("Error running migrations");

    start(pool).await
}

async fn start(pool: DbPool) -> std::io::Result<()> {
    HttpServer::new(move || {
        let mut app = App::new().wrap(
            Cors::default()
                .allow_any_origin()
                .allow_any_method()
                .allow_any_header()
        )
            .wrap(actix_web::middleware::Logger::new("%r responded %s in %D ms"))
            .data(pool.clone());

        let mut api_scope = web::scope("/api/v1/");

        // Services
        api_scope = api::users::register(api_scope);
        api_scope = api::auth::register(api_scope);

        app = app.service(api_scope);
        app
    })
        .bind(format!("{}:{}", &CONFIG.server.address, &CONFIG.server.port))?
        .run()
        .await
}

fn run_migrations (conn: &PgConnection) -> anyhow::Result<()> {
    embedded_migrations::run(conn)?;

    if User::find_by_email(&CONFIG.root_user_email, conn)?.is_none() {
        let mut root_user = User {
            id: 0,
            email: CONFIG.root_user_email.clone(),
            password: CONFIG.root_user_password.clone(),
            first_name: None,
            last_name: None,
            activated: true,
            updated_at: None,
            created_at: None
        };

        root_user.hash_password()?;

        root_user.insert(conn)?;
    }

    Ok(())
}