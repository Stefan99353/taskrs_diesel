use actix_web::{Scope, web};

mod controller;
mod actions;

pub fn register(scope: Scope) -> Scope {
    let mut scope = scope;
    let mut user_scope = web::scope("users")
        .wrap(crate::middleware::auth::Authentication);

    // Debug routes
    if cfg!(debug_assertions) {
        let debug_user_scope = web::scope("debug_users")
            .service(controller::all_users);

        scope = scope.service(debug_user_scope);
    }

    user_scope = user_scope.service(controller::add_user);

    scope.service(user_scope)
}