use actix_web::web;
use actix_web::{get, post, HttpResponse};

use taskrs_db::models::permission::PermissionColumns;
use taskrs_db::DbPool;

use crate::api::permissions::{ChangePermissionResult, UserPermissionsDto};
use crate::models::request_filter::RequestFilter;
use crate::models::user_token::TokenUser;
use crate::{permissions, utils};

use super::actions;

/// Returns a list of permissions
///
/// Needs permission `permission_get_all` for access
#[get("")]
pub async fn all_permissions(
    user: TokenUser,
    filter: web::Query<RequestFilter<PermissionColumns>>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let conn = utils::get_db_connection(pool.into_inner())?;
    let filter = filter.into_inner();

    // Check permission
    utils::has_permission(&user, &permissions::PERMISSION_GET_ALL, &conn)?;

    web::block(move || actions::get_all_permissions(filter, &conn))
        .await
        .map(|permissions| HttpResponse::Ok().json(permissions))
        .map_err(|e| {
            error!("{}", e);
            HttpResponse::InternalServerError()
                .body(e.to_string())
                .into()
        })
}

/// Grants permissions to a user
///
/// Needs permission `permission_grant` for access
#[post("/grant")]
pub async fn grant_permissions(
    user: TokenUser,
    new_permissions: web::Json<UserPermissionsDto>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let conn = utils::get_db_connection(pool.into_inner())?;
    let new_permissions = new_permissions.into_inner();

    // Check permission
    utils::has_permission(&user, &permissions::PERMISSION_GRANT, &conn)?;

    web::block(move || actions::grant_permissions(user.id, new_permissions, &conn))
        .await
        .map(|res| match res {
            ChangePermissionResult::Ok => HttpResponse::Ok().finish(),
            ChangePermissionResult::InvalidUser => HttpResponse::BadRequest().finish(),
        })
        .map_err(|e| {
            error!("{}", e);
            HttpResponse::InternalServerError()
                .body(e.to_string())
                .into()
        })
}

/// Revokes permissions from a user
///
/// Needs permission `permission_revoke` for access
#[post("/revoke")]
pub async fn revoke_permissions(
    user: TokenUser,
    old_permissions: web::Json<UserPermissionsDto>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let conn = utils::get_db_connection(pool.into_inner())?;
    let old_permissions = old_permissions.into_inner();

    // Check permission
    utils::has_permission(&user, &permissions::PERMISSION_REVOKE, &conn)?;

    web::block(move || actions::revoke_permissions(user.id, old_permissions, &conn))
        .await
        .map(|res| match res {
            ChangePermissionResult::Ok => HttpResponse::Ok().finish(),
            ChangePermissionResult::InvalidUser => HttpResponse::BadRequest().finish(),
        })
        .map_err(|e| {
            error!("{}", e);
            HttpResponse::InternalServerError()
                .body(e.to_string())
                .into()
        })
}

/// Sets permissions of a user
///
/// Needs permission `permission_set` for access
#[post("/set")]
pub async fn set_user_permissions(
    user: TokenUser,
    new_permissions: web::Json<UserPermissionsDto>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let conn = utils::get_db_connection(pool.into_inner())?;
    let new_permissions = new_permissions.into_inner();

    // Check permission
    utils::has_permission(&user, &permissions::PERMISSION_SET, &conn)?;

    web::block(move || actions::set_permissions(user.id, new_permissions, &conn))
        .await
        .map(|res| match res {
            ChangePermissionResult::Ok => HttpResponse::Ok().finish(),
            ChangePermissionResult::InvalidUser => HttpResponse::BadRequest().finish(),
        })
        .map_err(|e| {
            error!("{}", e);
            HttpResponse::InternalServerError()
                .body(e.to_string())
                .into()
        })
}
