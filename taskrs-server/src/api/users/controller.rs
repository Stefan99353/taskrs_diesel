use actix_web::{get, HttpResponse, post};
use actix_web::web;

use crate::utils;
use crate::db::DbPool;
use crate::db::user::User;
use crate::permissions;

use super::actions;

/// Returns a list of users
///
/// Permission: `user_get_all`
#[get("")]
pub async fn all_users(
    user: User,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let conn = utils::get_db_connection(pool.into_inner())?;

    // Check permission
    utils::has_permission(&user, permissions::USER_GET_ALL, &conn)?;

    web::block(move || actions::get_all_users(&conn))
        .await
        .map(|users| HttpResponse::Ok().json(users))
        .map_err(|e| {
            error!("{}", e);
            HttpResponse::InternalServerError().finish().into()
        })
}

/// Creates a new user
///
/// Permission: `user_create`
#[post("")]
pub async fn add_user(
    user: User,
    pool: web::Data<DbPool>,
    new_user: web::Json<User>,
) -> Result<HttpResponse, actix_web::Error> {
    let conn =utils::get_db_connection(pool.into_inner())?;
    let new_user = new_user.into_inner();

    // Check permission
    utils::has_permission(&user, permissions::USER_CREATE, &conn)?;

    // Create user
    web::block(move || actions::create_user(new_user, &conn))
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