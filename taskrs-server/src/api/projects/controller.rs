use actix_web::{delete, get, HttpResponse, post, put, web};

use crate::db::DbPool;
use crate::db::project::{ProjectColumns, Project};
use crate::db::user::User;
use crate::models::request_filter::RequestFilter;
use crate::permissions;
use crate::utils;

use super::actions;
use crate::models::create_entity_result::CreateEntityResult;

/// Returns a list of projects
///
/// Permission: `project_get_all`
///
#[get("")]
pub async fn all_projects(
    user: User,
    filter: web::Query<RequestFilter<ProjectColumns>>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let conn = utils::get_db_connection(pool.into_inner())?;
    let filter = filter.into_inner();

    // Check permission
    utils::has_permission(&user, permissions::PROJECT_GET_ALL, &conn)?;

    web::block(move || actions::get_all_projects(filter, &conn))
        .await
        .map(|categories| HttpResponse::Ok().json(categories))
        .map_err(|e| {
            error!("{}", e);
            HttpResponse::InternalServerError().body(e.to_string()).into()
        })
}

/// Creates a new projects
///
/// Permission: `project_create`
///
#[post("")]
pub async fn create_project(
    user: User,
    pool: web::Data<DbPool>,
    new_project: web::Json<Project>,
) -> Result<HttpResponse, actix_web::Error> {
    let conn = utils::get_db_connection(pool.into_inner())?;
    let new_project = new_project.into_inner();

    // Check permission
    utils::has_permission(&user, permissions::PROJECT_CREATE, &conn)?;

    // Create category
    web::block(move || actions::create_project(new_project, &conn))
        .await
        .map(|created_project| {
            match created_project {
                CreateEntityResult::Ok(project) => HttpResponse::Created().json(project),
                CreateEntityResult::Exists => HttpResponse::BadRequest().finish(),
            }
        })
        .map_err(|e| {
            error!("{}", e);
            HttpResponse::InternalServerError().body(e.to_string()).into()
        })
}

/// Delete a project
///
/// Permission: `project_delete`
///
#[delete("")]
pub async fn delete_project() -> Result<HttpResponse, actix_web::Error> {
    unimplemented!();
}

/// Update a project
///
/// Permission: `project_update`
///
#[put("")]
pub async fn update_project() -> Result<HttpResponse, actix_web::Error> {
    unimplemented!();
}