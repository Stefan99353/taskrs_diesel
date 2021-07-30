use std::sync::Arc;

use diesel::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use jsonwebtoken::{DecodingKey, TokenData, Validation};

use crate::CONFIG;
use crate::db::DbPool;
use crate::db::user::User;
use crate::models::user_token::UserToken;

pub fn decode_token(token: &str) -> jsonwebtoken::errors::Result<TokenData<UserToken>> {
    jsonwebtoken::decode::<UserToken>(token, &DecodingKey::from_secret(CONFIG.access_token_secret.as_bytes()), &Validation::default())
}

/// Get database connection from pool.
/// Returns Internal Server Error on Failure.
pub fn get_db_connection(pool: Arc<DbPool>) -> Result<PooledConnection<ConnectionManager<PgConnection>>, actix_web::Error> {
    pool.get().map_err(|err| {
        error!("{}", err);
        actix_web::HttpResponse::InternalServerError().finish().into()
    })
}

/// Checks if user has one of the provided permissions
/// Returns InternalServerError on DB error
/// Returns Unauthorized if no permission is matched
pub fn has_one_permission(
    user: &User,
    needed_permissions: Vec<&str>,
    conn: &PgConnection,
) -> Result<(), actix_web::Error> {
    debug!("Check if user {} has one permission: {:?}", user.id, needed_permissions);
    // Check cache
    if let Some(cache) = crate::PERMISSION_CACHE.read().unwrap().get(&user.id) {
        for needed_permission in &needed_permissions {
            if cache.contains(&needed_permission.to_string()) {
                // User has one permission
                debug!("User permission was found in cache");
                return Ok(());
            }
        }
    }

    // Permission not found in cache -> Update Cache
    let db_permissions = update_permission_cache_for_user(user, conn)
        .map_err(|err| {
            error!("{}", err);
            actix_web::HttpResponse::InternalServerError().finish()
        })?;

    // Check DB permissions
    for needed_permission in &needed_permissions {
        if db_permissions.contains(&needed_permission.to_string()) {
            // User has one permission
            debug!("User permission was found in database");
            return Ok(());
        }
    }

    // User doesn't have any needed permission
    debug!("User does not have any needed permission");
    Err(
        actix_web::HttpResponse::Unauthorized()
            .body(format!("Needs one permission of: {:?}", &needed_permissions))
            .into()
    )
}

/// Checks if user has all of the provided permissions
/// Returns InternalServerError on DB error
/// Returns Unauthorized if no permission is matched
pub fn has_all_permissions(
    user: &User,
    needed_permissions: Vec<&str>,
    conn: &PgConnection,
) -> Result<(), actix_web::Error> {
    debug!("Check if user {} has all permission: {:?}", user.id, needed_permissions);
    // Check cache
    if let Some(cache) = crate::PERMISSION_CACHE.read().unwrap().get(&user.id) {
        let mut has_all_permissions = true;
        for needed_permission in &needed_permissions {
            if !cache.contains(&needed_permission.to_string()) {
                // User doesn't have permission
                has_all_permissions = false;
                break;
            }
        }

        if has_all_permissions {
            // User has all permissions
            debug!("User permissions found in cache");
            return Ok(());
        }
    }

    // Permission not found in cache -> Update Cache
    let db_permissions = update_permission_cache_for_user(user, conn)
        .map_err(|err| {
            error!("{}", err);
            actix_web::HttpResponse::InternalServerError().finish()
        })?;

    // Check DB permissions
    let mut has_all_permissions = true;
    for needed_permission in &needed_permissions {
        if !db_permissions.contains(&needed_permission.to_string()) {
            // User doesn't have permission
            has_all_permissions = false;
            break;
        }
    }

    if has_all_permissions {
        // User has all permissions
        debug!("User permissions found in database");
        return Ok(());
    }

    // User doesn't have all needed permissions
    debug!("User does not have all permissions");
    Err(
        actix_web::HttpResponse::Unauthorized()
            .body(format!("Needs all permissions of: {:?}", &needed_permissions))
            .into()
    )
}

/// Checks if user has permission
/// Returns InternalServerError on DB error
/// Returns Unauthorized if no permission is matched
pub fn has_permission(
    user: &User,
    needed_permission: &str,
    conn: &PgConnection,
) -> Result<(), actix_web::Error> {
    debug!("Check if user {} has permission: {}", user.id, needed_permission);
    // Check cache
    if let Some(cache) = crate::PERMISSION_CACHE.read().unwrap().get(&user.id) {
        if cache.contains(&needed_permission.to_string()) {
            // User has permission
            debug!("User permission was found in cache");
            return Ok(());
        }
    }

    // Permission not found in cache -> Update Cache
    let db_permissions = update_permission_cache_for_user(user, conn)
        .map_err(|err| {
            error!("{}", err);
            actix_web::HttpResponse::InternalServerError().finish()
        })?;

    // Check DB permissions
    if db_permissions.contains(&needed_permission.to_string()) {
        // User has permission
        debug!("User permission was found in database");
        return Ok(());
    }

    // User doesn't have permission
    debug!("User does not have permission");
    Err(
        actix_web::HttpResponse::Unauthorized()
            .body(format!("Needs permission: {:?}", &needed_permission))
            .into()
    )
}

fn update_permission_cache_for_user(
    user: &User,
    conn: &PgConnection,
) -> Result<Vec<String>, diesel::result::Error> {
    use crate::db::schema::{permissions, user_permissions};

    debug!("Updating permission cache for user {}", user.id);
    let db_permissions: Vec<String> = permissions::table.inner_join(user_permissions::table)
        .filter(user_permissions::user_id.eq(user.id))
        .select(permissions::name)
        .load::<String>(conn)?;

    // Update cache
    let mut cache = crate::PERMISSION_CACHE.write().unwrap();
    cache.insert(user.id, db_permissions.clone());

    Ok(db_permissions)
}