use actix_web::{Scope, web};

mod controller;
mod actions;

pub fn register(scope: Scope) -> Scope {
    let scope = scope;
    let mut user_scope = web::scope("users")
        .wrap(crate::middleware::auth::Authentication);

    // Debug routes
    if cfg!(debug_assertions) {}

    user_scope = user_scope.service(controller::all_users);
    user_scope = user_scope.service(controller::add_user);

    scope.service(user_scope)
}