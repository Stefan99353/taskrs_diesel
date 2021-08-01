use actix_web::{Scope, web};

mod controller;
mod actions;

pub fn register(scope: Scope) -> Scope {
    let mut permission_scope = web::scope("permissions")
        .wrap(crate::middleware::auth::Authentication);

    // Debug routes
    if cfg!(debug_assertions) {}

    permission_scope = permission_scope.service(controller::all_permissions);

    scope.service(permission_scope)
}