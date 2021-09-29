use actix_web::{delete, get, HttpResponse, post, web, put};

use crate::db::category::{Category, CategoryColumns};
use crate::db::DbPool;
use crate::db::user::User;
use crate::models::create_entity_result::CreateEntityResult;
use crate::models::delete_entity::{DeleteEntityParams, DeleteEntityResult};
use crate::models::request_filter::RequestFilter;
use crate::permissions;
use crate::utils;

use super::actions;

/// Returns a list of categories
///
/// Permission: `category_get_all`
///
#[get("")]
pub async fn all_categories(
    user: User,
    filter: web::Query<RequestFilter<CategoryColumns>>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let conn = utils::get_db_connection(pool.into_inner())?;
    let filter = filter.into_inner();

    // Check permission
    utils::has_permission(&user, permissions::CATEGORY_GET_ALL, &conn)?;

    web::block(move || actions::get_all_categories(filter, &conn))
        .await
        .map(|categories| HttpResponse::Ok().json(categories))
        .map_err(|e| {
            error!("{}", e);
            HttpResponse::InternalServerError().body(e.to_string()).into()
        })
}

/// Creates a new category
///
/// Permission: `category_create`
#[post("")]
pub async fn create_category(
    user: User,
    pool: web::Data<DbPool>,
    new_category: web::Json<Category>,
) -> Result<HttpResponse, actix_web::Error> {
    let conn = utils::get_db_connection(pool.into_inner())?;
    let new_category = new_category.into_inner();

    // Check permission
    utils::has_permission(&user, permissions::CATEGORY_CREATE, &conn)?;

    // Create category
    web::block(move || actions::create_category(new_category, &conn))
        .await
        .map(|created_category| {
            match created_category {
                CreateEntityResult::Ok(category) => HttpResponse::Created().json(category),
                CreateEntityResult::Exists => HttpResponse::BadRequest().finish(),
            }
        })
        .map_err(|e| {
            error!("{}", e);
            HttpResponse::InternalServerError().body(e.to_string()).into()
        })
}

/// Delete a category
///
/// Permission: `category_delete`
#[delete("")]
pub async fn delete_category(
    params: web::Query<DeleteEntityParams>,
    user: User,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let conn = utils::get_db_connection(pool.into_inner())?;
    let params = params.into_inner();

    // Check permission
    utils::has_permission(&user, permissions::CATEGORY_DELETE, &conn)?;

    // Delete category
    web::block(move || actions::delete_category(params, &conn))
        .await
        .map(|created_category| {
            match created_category {
                DeleteEntityResult::Ok => HttpResponse::Ok().finish(),
                DeleteEntityResult::NotFound => HttpResponse::NotFound().finish(),
                DeleteEntityResult::Referenced(references) => HttpResponse::BadRequest().json(references),
            }
        })
        .map_err(|e| {
            error!("{}", e);
            HttpResponse::InternalServerError().body(e.to_string()).into()
        })
}

/// Update a category
///
/// Permission: `category_update`
#[put("")]
pub async fn update_category(
    user: User,
    pool: web::Data<DbPool>,
    category: web::Json<Category>,
) -> Result<HttpResponse, actix_web::Error> {
    let conn = utils::get_db_connection(pool.into_inner())?;
    let category = category.into_inner();

    // Check permission
    utils::has_permission(&user, permissions::CATEGORY_UPDATE, &conn)?;

    // Update category
    web::block(move || actions::update_category(category, &conn))
        .await
        .map(|updated_category| {
            match updated_category {
                Some(category) => HttpResponse::Ok().json(category),
                None => HttpResponse::NotFound().finish()
            }
        })
        .map_err(|e| {
            error!("{}", e);
            HttpResponse::InternalServerError().body(e.to_string()).into()
        })
}