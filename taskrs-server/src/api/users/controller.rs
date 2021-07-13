use actix_web::{get, HttpResponse, post};
use actix_web::web;

use crate::api::get_db_connection;
use crate::db::DbPool;
use crate::db::user::User;

use super::actions;

#[get("")]
pub async fn all_users(
    pool: web::Data<DbPool>
) -> Result<HttpResponse, actix_web::Error> {
    let conn = get_db_connection(pool.into_inner())?;

    web::block(move || actions::get_all_users(&conn))
        .await
        .map(|users| HttpResponse::Ok().json(users))
        .map_err(|e| {
            error!("{}", e);
            HttpResponse::InternalServerError().finish().into()
        })
}

#[post("")]
pub async fn add_user(
    pool: web::Data<DbPool>,
    user: web::Json<User>,
) -> Result<HttpResponse, actix_web::Error> {
    let conn = get_db_connection(pool.into_inner())?;
    let user = user.into_inner();

    // Create user
    web::block(move || actions::create_user(user, &conn))
        .await
        .map(|created_user| {
            match created_user {
                CreateUserResult::Ok(user) => HttpResponse::Created().json(user),
                CreateUserResult::Exists => HttpResponse::BadRequest().finish(),
            }
        })
        .map_err(|e| {
            error!("{}", e);
            HttpResponse::InternalServerError().finish().into()
        })
}

pub enum CreateUserResult {
    Ok(User),
    Exists,
}