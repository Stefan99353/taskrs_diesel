use actix_web::{Scope, web};
use serde::{Deserialize, Serialize};

mod controller;
mod actions;

pub fn register(scope: Scope) -> Scope {
    let mut auth_scope = web::scope("auth");

    // Debug routes
    if cfg!(debug_assertions) {}

    auth_scope = auth_scope.service(controller::login);
    auth_scope = auth_scope.service(controller::logout);
    auth_scope = auth_scope.service(controller::refresh_token);
    auth_scope = auth_scope.service(controller::revoke_token);

    scope.service(auth_scope)
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserTokensDto {
    pub access_token: String,
    pub refresh_token: String,
}