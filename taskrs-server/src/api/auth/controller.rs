use actix_web::{HttpResponse, post, web};

use crate::db::DbPool;
use crate::db::user::{SimpleUser, User};
use crate::permissions;
use crate::utils;

use super::actions;

#[post("/login")]
pub async fn login(
    user: web::Json<SimpleUser>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let conn = utils::get_db_connection(pool.into_inner())?;
    let user = user.into_inner();

    // Login user
    web::block(move || actions::login(user, &conn))
        .await
        .map(|tokens| {
            match tokens {
                Some(token) => HttpResponse::Ok().json(token),
                None => HttpResponse::BadRequest().finish(),
            }
        })
        .map_err(|e| {
            error!("{}", e);
            HttpResponse::InternalServerError().body(e.to_string()).into()
        })
}

#[post("/logout")]
pub async fn logout(
    ref_token: web::Json<String>,
    user: User,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let conn = utils::get_db_connection(pool.into_inner())?;
    let ref_token = ref_token.into_inner();

    // Logout
    web::block(move || actions::logout(ref_token, user, &conn))
        .await
        .map(|_| {
            HttpResponse::Ok().finish()
        })
        .map_err(|e| {
            error!("{}", e);
            HttpResponse::InternalServerError().body(e.to_string()).into()
        })
}

#[post("/token")]
pub async fn refresh_token(
    ref_token: web::Json<String>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let conn = utils::get_db_connection(pool.into_inner())?;
    let ref_token = ref_token.into_inner();

    web::block(move || actions::refresh_token(&ref_token, &conn))
        .await
        .map(|token| {
            match token {
                Some(token) => HttpResponse::Ok().json(token),
                None => HttpResponse::Forbidden().finish(),
            }
        })
        .map_err(|e| {
            error!("{}", e);
            HttpResponse::InternalServerError().body(e.to_string()).into()
        })
}

#[post("/token/revoke")]
pub async fn revoke_token(
    ref_token: web::Json<String>,
    request_user: User,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let conn = utils::get_db_connection(pool.into_inner())?;
    let ref_token = ref_token.into_inner();

    // Check permission
    utils::has_permission(&request_user, permissions::AUTH_REVOKE_REFRESH_TOKEN, &conn)?;

    web::block(move || actions::revoke_token(&ref_token, &conn))
        .await
        .map_err(|e| {
            error!("{}", e);
            HttpResponse::InternalServerError().body(e.to_string())
        })?;

    Ok(HttpResponse::Ok().finish())
}