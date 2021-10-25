use std::sync::Arc;

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use jsonwebtoken::{DecodingKey, TokenData, Validation};

use taskrs_db::models::permission::Permission;
use taskrs_db::{DbConnection, DbPool};

use crate::models::user_token::{TokenUser, UserToken};

/// Decodes and validates the JWT
/// Returns Error if token is invalid
pub fn decode_token(
    token: &str,
    secret: &str,
) -> jsonwebtoken::errors::Result<TokenData<UserToken>> {
    jsonwebtoken::decode::<UserToken>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
}

/// Get database connection from pool.
/// Returns Internal Server Error on Failure.
pub fn get_db_connection(
    pool: Arc<DbPool>,
) -> Result<PooledConnection<ConnectionManager<DbConnection>>, actix_web::Error> {
    pool.get().map_err(|err| {
        error!("{}", err);
        actix_web::HttpResponse::InternalServerError()
            .finish()
            .into()
    })
}

/// Checks if user has one of the provided permissions
/// Returns InternalServerError on DB error
/// Returns Forbidden if no permission is matched
pub fn has_one_permission(
    user: &TokenUser,
    needed_permissions: Vec<&Permission>,
    conn: &PgConnection,
) -> Result<(), actix_web::Error> {
    debug!(
        "Check if user {} has one permission: {:?}",
        user.id,
        needed_permissions
            .iter()
            .map(|x| &x.name)
            .collect::<Vec<&String>>()
    );
    // Check cache
    if let Some(cache) = crate::PERMISSION_CACHE.read().unwrap().get(&user.id) {
        for needed_permission in &needed_permissions {
            if cache.contains(&needed_permission.name) {
                // User has one permission
                debug!("User permission was found in cache");
                return Ok(());
            }
        }
    }

    // Permission not found in cache -> Update Cache
    let db_permissions = update_permission_cache_for_user(user.id, conn).map_err(|err| {
        error!("{}", err);
        actix_web::HttpResponse::InternalServerError().finish()
    })?;

    // Check DB permissions
    for needed_permission in &needed_permissions {
        if db_permissions.contains(&needed_permission.name) {
            // User has one permission
            debug!("User permission was found in database");
            return Ok(());
        }
    }

    // User doesn't have any needed permission
    debug!("User does not have any needed permission");
    Err(actix_web::HttpResponse::Forbidden()
        .body(format!(
            "Needs one permission of: {:?}",
            needed_permissions
                .iter()
                .map(|x| &x.name)
                .collect::<Vec<&String>>()
        ))
        .into())
}

/// Checks if user has all of the provided permissions
/// Returns InternalServerError on DB error
/// Returns Forbidden if no permission is matched
pub fn has_all_permissions(
    user: &TokenUser,
    needed_permissions: Vec<&Permission>,
    conn: &PgConnection,
) -> Result<(), actix_web::Error> {
    debug!(
        "Check if user {} has all permission: {:?}",
        user.id,
        needed_permissions
            .iter()
            .map(|x| &x.name)
            .collect::<Vec<&String>>()
    );
    // Check cache
    if let Some(cache) = crate::PERMISSION_CACHE.read().unwrap().get(&user.id) {
        let mut has_all_permissions = true;
        for needed_permission in &needed_permissions {
            if !cache.contains(&needed_permission.name) {
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
    let db_permissions = update_permission_cache_for_user(user.id, conn).map_err(|err| {
        error!("{}", err);
        actix_web::HttpResponse::InternalServerError().finish()
    })?;

    // Check DB permissions
    let mut has_all_permissions = true;
    for needed_permission in &needed_permissions {
        if !db_permissions.contains(&needed_permission.name) {
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
    Err(actix_web::HttpResponse::Forbidden()
        .body(format!(
            "Needs all permissions of: {:?}",
            needed_permissions
                .iter()
                .map(|x| &x.name)
                .collect::<Vec<&String>>()
        ))
        .into())
}

/// Checks if user has permission
/// Returns InternalServerError on DB error
/// Returns Forbidden if no permission is matched
pub fn has_permission(
    user: &TokenUser,
    needed_permission: &Permission,
    conn: &PgConnection,
) -> Result<(), actix_web::Error> {
    debug!(
        "Check if user {} has permission: {}",
        user.id, needed_permission.name
    );
    // Check cache
    if let Some(cache) = crate::PERMISSION_CACHE.read().unwrap().get(&user.id) {
        if cache.contains(&needed_permission.name) {
            // User has permission
            debug!("User permission was found in cache");
            return Ok(());
        }
    }

    // Permission not found in cache -> Update Cache
    let db_permissions = update_permission_cache_for_user(user.id, conn).map_err(|err| {
        error!("{}", err);
        actix_web::HttpResponse::InternalServerError().finish()
    })?;

    // Check DB permissions
    if db_permissions.contains(&needed_permission.name) {
        // User has permission
        debug!("User permission was found in database");
        return Ok(());
    }

    // User doesn't have permission
    debug!("User does not have permission");
    Err(actix_web::HttpResponse::Forbidden()
        .body(format!("Needs permission: {:?}", &needed_permission.name))
        .into())
}

/// Update the permission cache for a single user
pub fn update_permission_cache_for_user(
    user_id: i32,
    conn: &PgConnection,
) -> Result<Vec<String>, diesel::result::Error> {
    use taskrs_db::schema::{permissions, user_permissions};

    debug!("Updating permission cache for user {}", user_id);
    let db_permissions: Vec<String> = permissions::table
        .inner_join(user_permissions::table)
        .filter(user_permissions::user_id.eq(user_id))
        .select(permissions::name)
        .load::<String>(conn)?;

    // Update cache
    let mut cache = crate::PERMISSION_CACHE.write().unwrap();
    cache.insert(user_id, db_permissions.clone());

    Ok(db_permissions)
}
