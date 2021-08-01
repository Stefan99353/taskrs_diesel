use actix_web::{delete, get, HttpResponse, post};
use actix_web::web;

use crate::{permissions, utils};
use crate::db::DbPool;
use crate::db::user::User;

use super::actions;
use crate::db::permission::UserPermission;

#[get("")]
pub async fn all_permissions(
    user: User,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let conn = utils::get_db_connection(pool.into_inner())?;

    // Check permission
    utils::has_permission(&user, permissions::PERMISSION_GET_ALL, &conn)?;

    web::block(move || actions::get_all_permissions(&conn))
        .await
        .map(|permissions| HttpResponse::Ok().json(permissions))
        .map_err(|e| {
            error!("{}", e);
            HttpResponse::InternalServerError().finish().into()
        })
}

#[post("")]
pub async fn grant_permissions(
    user: User,
    new_permissions: web::Json<Vec<UserPermission>>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let conn = utils::get_db_connection(pool.into_inner())?;
    let new_permissions = new_permissions.into_inner();

    // Check permission
    utils::has_permission(&user, permissions::PERMISSION_GRANT, &conn)?;

    web::block(move || actions::grant_permissions(&user, new_permissions, &conn))
        .await
        .map(|_| HttpResponse::Ok().finish())
        .map_err(|e| {
            error!("{}", e);
            HttpResponse::InternalServerError().finish().into()
        })
}

#[delete("")]
pub async fn revoke_permission(
    user: User,
    old_permissions: web::Json<Vec<UserPermission>>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let conn = utils::get_db_connection(pool.into_inner())?;
    let old_permissions = old_permissions.into_inner();

    // Check permission
    utils::has_permission(&user, permissions::PERMISSION_REVOKE, &conn)?;

    web::block(move || actions::revoke_permissions(&user, old_permissions, &conn))
        .await
        .map(|_| HttpResponse::Ok().finish())
        .map_err(|e| {
            error!("{}", e);
            HttpResponse::InternalServerError().finish().into()
        })
}

#[post("/set")]
pub async fn set_user_permissions(
    user: User,
    new_permissions: web::Json<Vec<UserPermission>>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let conn = utils::get_db_connection(pool.into_inner())?;
    let new_permissions = new_permissions.into_inner();

    // Check permission
    utils::has_permission(&user, permissions::PERMISSION_SET, &conn)?;

    web::block(move || actions::set_permissions(&user, new_permissions, &conn))
        .await
        .map(|_| HttpResponse::Ok().finish())
        .map_err(|e| {
            error!("{}", e);
            HttpResponse::InternalServerError().finish().into()
        })
}