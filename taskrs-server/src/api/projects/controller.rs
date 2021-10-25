use actix_web::{delete, get, HttpResponse, post, put, web};

use taskrs_db::DbPool;
use taskrs_db::models::project::{Project, ProjectColumns};

use crate::models::create_entity_result::CreateEntityResult;
use crate::models::delete_entity::{DeleteEntityParams, DeleteEntityResult};
use crate::models::request_filter::RequestFilter;
use crate::models::user_token::TokenUser;
use crate::permissions;
use crate::utils;

use super::actions;

/// Returns a list of projects
///
/// Permission: `project_get_all`
///
#[get("")]
pub async fn all_projects(
    user: TokenUser,
    filter: web::Query<RequestFilter<ProjectColumns>>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let conn = utils::get_db_connection(pool.into_inner())?;
    let filter = filter.into_inner();

    // Check permission
    utils::has_permission(&user, &permissions::PROJECT_GET_ALL, &conn)?;

    web::block(move || actions::get_all_projects(filter, &conn))
        .await
        .map(|categories| HttpResponse::Ok().json(categories))
        .map_err(|e| {
            error!("{}", e);
            HttpResponse::InternalServerError()
                .body(e.to_string())
                .into()
        })
}

/// Creates a new projects
///
/// Permission: `project_create`
///
#[post("")]
pub async fn create_project(
    user: TokenUser,
    pool: web::Data<DbPool>,
    new_project: web::Json<Project>,
) -> Result<HttpResponse, actix_web::Error> {
    let conn = utils::get_db_connection(pool.into_inner())?;
    let mut new_project = new_project.into_inner();

    // Check permission
    utils::has_permission(&user, &permissions::PROJECT_CREATE, &conn)?;

    // Set project creator
    new_project.creator_id = Some(user.id);

    // Create category
    web::block(move || actions::create_project(new_project, &conn))
        .await
        .map(|created_project| match created_project {
            CreateEntityResult::Ok(project) => HttpResponse::Created().json(project),
            CreateEntityResult::Exists => HttpResponse::BadRequest().finish(),
        })
        .map_err(|e| {
            error!("{}", e);
            HttpResponse::InternalServerError()
                .body(e.to_string())
                .into()
        })
}

/// Delete a project
///
/// Permission: `project_delete`
///
#[delete("")]
pub async fn delete_project(
    params: web::Query<DeleteEntityParams>,
    user: TokenUser,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let conn = utils::get_db_connection(pool.into_inner())?;
    let params = params.into_inner();

    // Check permission
    utils::has_permission(&user, &permissions::PROJECT_DELETE, &conn)?;

    // Delete project
    web::block(move || actions::delete_project(params, &conn))
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

/// Update a project
///
/// Permission: `project_update`
///
#[put("")]
pub async fn update_project(
    project: web::Json<Project>,
    user: TokenUser,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let conn = utils::get_db_connection(pool.into_inner())?;
    let project = project.into_inner();

    // Check permission
    utils::has_permission(&user, &permissions::PROJECT_UPDATE, &conn)?;

    // Update project
    web::block(move || actions::update_project(project, &conn))
        .await
        .map(|updated_project| match updated_project {
            Some(project) => HttpResponse::Ok().json(project),
            None => HttpResponse::NotFound().finish(),
        })
        .map_err(|e| {
            error!("{}", e);
            HttpResponse::InternalServerError()
                .body(e.to_string())
                .into()
        })
}
