use actix_web::{Scope, web};
use serde::{Deserialize, Serialize};

mod controller;
mod actions;

pub fn register(scope: Scope) -> Scope {
    let mut permission_scope = web::scope("permissions")
        .wrap(crate::middleware::auth::Authentication);

    // Debug routes
    if cfg!(debug_assertions) {}

    permission_scope = permission_scope
        .service(controller::all_permissions)
        .service(controller::grant_permissions)
        .service(controller::revoke_permission)
        .service(controller::set_user_permissions);

    scope.service(permission_scope)
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserPermissionsDto {
    pub user_id: i32,
    pub permission_ids: Vec<i32>,
}

pub enum ChangePermissionResult {
    Ok,
    InvalidUser,
}