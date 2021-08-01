use diesel::dsl::count;
use diesel::PgConnection;
use diesel::prelude::*;

use crate::api::permissions::ChangePermissionResult;
use crate::db::permission::{NewUserPermission, Permission};
use crate::db::user::User;
use crate::utils::update_permission_cache_for_user;

use super::UserPermissionsDto;

pub fn get_all_permissions(conn: &PgConnection) -> Result<Vec<Permission>, diesel::result::Error> {
    use crate::db::schema::permissions::dsl::*;
    permissions.load(conn)
}

pub fn grant_permissions(
    user: &User,
    new_permissions: UserPermissionsDto,
    conn: &PgConnection,
) -> Result<ChangePermissionResult, diesel::result::Error> {
    use crate::db::schema::{user_permissions, users};

    let count = users::table
        .select(count(users::id))
        .filter(users::id.eq(&new_permissions.user_id))
        .first::<i64>(conn)?;

    // User does not exist
    if count != 1 { return Ok(ChangePermissionResult::InvalidUser); }

    let current_permissions: Vec<i32> = user_permissions::table
        .select(user_permissions::permission_id)
        .filter(user_permissions::user_id.eq(&new_permissions.user_id))
        .load::<i32>(conn)?;

    let new_permissions = new_permissions.permission_ids
        .iter()
        .filter(|permission_id| {
            !current_permissions.contains(permission_id)
        })
        .map(|permission_id| {
            NewUserPermission {
                user_id: new_permissions.user_id,
                permission_id: *permission_id,
            }
        })
        .collect::<Vec<NewUserPermission>>();

    diesel::insert_into(user_permissions::table)
        .values(&new_permissions)
        .execute(conn)?;

    update_permission_cache_for_user(user, conn)?;

    Ok(ChangePermissionResult::Ok)
}

pub fn revoke_permissions(
    user: &User,
    old_permissions: UserPermissionsDto,
    conn: &PgConnection,
) -> Result<ChangePermissionResult, diesel::result::Error> {
    use crate::db::schema::{user_permissions, users};

    let count = users::table
        .select(count(users::id))
        .filter(users::id.eq(&old_permissions.user_id))
        .first::<i64>(conn)?;

    // User does not exist
    if count != 1 { return Ok(ChangePermissionResult::InvalidUser); }

    diesel::delete(user_permissions::table
        .filter(
            user_permissions::user_id
                .eq(&old_permissions.user_id)
                .and(user_permissions::permission_id.eq_any(old_permissions.permission_ids))
        )
    )
        .execute(conn)?;

    update_permission_cache_for_user(user, conn)?;

    Ok(ChangePermissionResult::Ok)
}

pub fn set_permissions(
    user: &User,
    new_permissions: UserPermissionsDto,
    conn: &PgConnection,
) -> Result<ChangePermissionResult, diesel::result::Error> {
    use crate::db::schema::{user_permissions, users};

    let count = users::table
        .select(count(users::id))
        .filter(users::id.eq(&new_permissions.user_id))
        .first::<i64>(conn)?;

    // User does not exist
    if count != 1 { return Ok(ChangePermissionResult::InvalidUser); }

    diesel::delete(user_permissions::table
        .filter(
            user_permissions::user_id.eq(&new_permissions.user_id)
        )
    )
        .execute(conn)?;

    let new_permissions = new_permissions.permission_ids
        .iter()
        .map(|permission_id| {
            NewUserPermission {
                user_id: new_permissions.user_id,
                permission_id: *permission_id,
            }
        })
        .collect::<Vec<NewUserPermission>>();

    diesel::insert_into(user_permissions::table)
        .values(&new_permissions)
        .execute(conn)?;

    update_permission_cache_for_user(user, conn)?;

    Ok(ChangePermissionResult::Ok)
}
