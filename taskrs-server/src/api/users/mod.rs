use actix_web::{web, Scope};

mod actions;
mod controller;

pub fn register(scope: Scope) -> Scope {
    let mut user_scope = web::scope("users").wrap(crate::middleware::auth::Authentication);

    // Debug routes
    if cfg!(debug_assertions) {}

    user_scope = user_scope
        .service(controller::all_users)
        .service(controller::create_user)
        .service(controller::delete_user)
        .service(controller::update_user);

    scope.service(user_scope)
}
