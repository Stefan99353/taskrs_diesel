use actix_web::{Scope, web};

mod controller;
mod actions;

pub fn register(scope: Scope) -> Scope {
    let mut category_scope = web::scope("categories")
        .wrap(crate::middleware::auth::Authentication);

    // Debug routes
    if cfg!(debug_assertions) {}

    category_scope = category_scope
        .service(controller::all_categories)
        .service(controller::create_category)
        .service(controller::delete_category)
        .service(controller::update_category);

    scope.service(category_scope)
}
