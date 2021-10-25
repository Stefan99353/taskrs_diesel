use actix_web::{Scope, web};

mod actions;
mod controller;

pub fn register(scope: Scope) -> Scope {
    let mut project_scope = web::scope("projects").wrap(crate::middleware::auth::Authentication);

    // Debug routes
    if cfg!(debug_assertions) {}

    project_scope = project_scope
        .service(controller::all_projects)
        .service(controller::create_project)
        .service(controller::delete_project)
        .service(controller::update_project);

    scope.service(project_scope)
}
