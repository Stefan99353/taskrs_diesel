#![allow(clippy::module_inception)]

pub use auth::*;
pub use categories::*;
pub use permissions::*;
pub use projects::*;
use taskrs_db::models::permission::Permission;
pub use users::*;

mod auth;
mod categories;
mod permissions;
mod projects;
mod users;

pub fn all_permissions() -> Vec<&'static Permission> {
    vec![
        &auth::AUTH_REVOKE_REFRESH_TOKEN,
        &categories::CATEGORY_GET_ALL,
        &categories::CATEGORY_CREATE,
        &categories::CATEGORY_DELETE,
        &categories::CATEGORY_UPDATE,
        &permissions::PERMISSION_GET_ALL,
        &permissions::PERMISSION_SET,
        &permissions::PERMISSION_GRANT,
        &permissions::PERMISSION_REVOKE,
        &projects::PROJECT_GET_ALL,
        &projects::PROJECT_CREATE,
        &projects::PROJECT_DELETE,
        &projects::PROJECT_UPDATE,
        &users::USER_GET_ALL,
        &users::USER_CREATE,
        &users::USER_DELETE,
        &users::USER_UPDATE,
    ]
}
