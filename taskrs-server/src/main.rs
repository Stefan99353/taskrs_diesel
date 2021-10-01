#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

use std::collections::HashMap;
use std::sync::RwLock;

use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use diesel::{PgConnection, QueryDsl};
use diesel::prelude::*;
use dotenv::dotenv;

use crate::db::DbPool;
use crate::db::permission::{NewUserPermission, Permission};
use crate::db::user::User;

mod config;
pub mod db;
mod api;
mod middleware;
mod models;
pub mod utils;
pub mod permissions;

lazy_static! {
    static ref CONFIG: crate::config::Config = crate::config::Config::new().expect("Error reading config");
    static ref PERMISSION_CACHE: RwLock<HashMap<i32, Vec<String>>> = RwLock::new(HashMap::new());
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Logger, .env, Database, Migrations
    log4rs::init_file("config/log.yml", Default::default()).unwrap();
    dotenv().ok();
    let db_connection = db::connect_database().await.unwrap();


    Ok(())

    // let conn = pool.get().expect("Couldn't get db connection from pool");
    // run_migrations(&conn).expect("Error running migrations_diesel");

    // start(pool).await
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
        api_scope = api::permissions::register(api_scope);
        api_scope = api::categories::register(api_scope);

        app = app.service(api_scope);
        app
    })
        .bind(format!("{}:{}", &CONFIG.server.address, &CONFIG.server.port))?
        .run()
        .await
}

fn run_migrations(conn: &PgConnection) -> anyhow::Result<()> {
    embedded_migrations::run(conn)?;

    let mut root_user = User::find_by_email(&CONFIG.root_user_email, conn)?;

    if root_user.is_none() {
        debug!("Seeding root user '{}'", &CONFIG.root_user_email);

        let mut new_root_user = User {
            id: 0,
            email: CONFIG.root_user_email.clone(),
            password: CONFIG.root_user_password.clone(),
            first_name: None,
            last_name: None,
            activated: true,
            updated_at: None,
            created_at: None,
        };

        new_root_user.hash_password()?;

        root_user = Some(new_root_user.insert(conn)?);
    }

    if let (Some(root_user), true) = (root_user, CONFIG.seed_root_permissions) {
        debug!("Seeding permissions for root user");
        use db::schema::{permissions, users, user_permissions};

        let root_permissions: Vec<Permission> = permissions::table
            .filter(permissions::id.ne_all(
                user_permissions::table
                    .inner_join(users::table)
                    .filter(users::email.eq(&CONFIG.root_user_email))
                    .select(user_permissions::permission_id)
            ))
            .load::<Permission>(conn)?;

        let new_root_permissions = root_permissions
            .into_iter()
            .map(|per| {
                NewUserPermission {
                    user_id: root_user.id,
                    permission_id: per.id,
                }
            })
            .collect::<Vec<NewUserPermission>>();

        diesel::insert_into(user_permissions::table)
            .values(new_root_permissions)
            .execute(conn)?;
    }

    Ok(())
}

