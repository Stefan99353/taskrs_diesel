use actix_web::web;
use actix_web::{delete, get, post, put, HttpResponse};

use crate::db::user::{User, UserColumns};
use crate::db::DbPool;
use crate::models::create_entity_result::CreateEntityResult;
use crate::models::request_filter::RequestFilter;
use crate::permissions;
use crate::utils;

use super::actions;
use crate::models::delete_entity::{DeleteEntityParams, DeleteEntityResult};

/// Returns a list of users
///
/// Permission: `user_get_all`
#[get("")]
pub async fn all_users(
    user: User,
    filter: web::Query<RequestFilter<UserColumns>>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let conn = utils::get_db_connection(pool.into_inner())?;
    let filter = filter.into_inner();

    // Check permission
    utils::has_permission(&user, permissions::USER_GET_ALL, &conn)?;

    web::block(move || actions::get_all_users(filter, &conn))
        .await
        .map(|users| HttpResponse::Ok().json(users))
        .map_err(|e| {
            error!("{}", e);
            HttpResponse::InternalServerError()
                .body(e.to_string())
                .into()
        })
}

/// Creates a new user
///
/// Permission: `user_create`
#[post("")]
pub async fn create_user(
    user: User,
    pool: web::Data<DbPool>,
    new_user: web::Json<User>,
) -> Result<HttpResponse, actix_web::Error> {
    let conn = utils::get_db_connection(pool.into_inner())?;
    let new_user = new_user.into_inner();

    // Check permission
    utils::has_permission(&user, permissions::USER_CREATE, &conn)?;

    // Create user
    web::block(move || actions::create_user(new_user, &conn))
        .await
        .map(|created_user| match created_user {
            CreateEntityResult::Ok(user) => HttpResponse::Created().json(user),
            CreateEntityResult::Exists => HttpResponse::BadRequest().finish(),
        })
        .map_err(|e| {
            error!("{}", e);
            HttpResponse::InternalServerError()
                .body(e.to_string())
                .into()
        })
}

/// Delete a user
///
/// Permission: `user_delete`
#[delete("")]
pub async fn delete_user(
    params: web::Query<DeleteEntityParams>,
    user: User,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let conn = utils::get_db_connection(pool.into_inner())?;
    let params = params.into_inner();

    // Check permission
    utils::has_permission(&user, permissions::USER_DELETE, &conn)?;

    // Delete user
    web::block(move || actions::delete_user(params, &conn))
        .await
        .map(|result| match result {
            DeleteEntityResult::Ok => HttpResponse::Ok().finish(),
            DeleteEntityResult::NotFound => HttpResponse::NotFound().finish(),
            DeleteEntityResult::Referenced(references) => {
                HttpResponse::BadRequest().json(references)
            }
        })
        .map_err(|e| {
            error!("{}", e);
            HttpResponse::InternalServerError()
                .body(e.to_string())
                .into()
        })
}

/// Update a user
///
/// Permission: `user_update`
#[put("")]
pub async fn update_user(
    user: User,
    pool: web::Data<DbPool>,
    updated_user: web::Json<User>,
) -> Result<HttpResponse, actix_web::Error> {
    let conn = utils::get_db_connection(pool.into_inner())?;
    let updated_user = updated_user.into_inner();

    // Check permission
    utils::has_permission(&user, permissions::USER_UPDATE, &conn)?;

    // Update user
    web::block(move || actions::update_user(updated_user, &conn))
        .await
        .map(|updated_user| match updated_user {
            Some(user) => HttpResponse::Ok().json(user),
            None => HttpResponse::NotFound().finish(),
        })
        .map_err(|e| {
            error!("{}", e);
            HttpResponse::InternalServerError()
                .body(e.to_string())
                .into()
        })
}
