use diesel::PgConnection;
use diesel::prelude::*;

use crate::db::permission::{NewUserPermission, Permission, UserPermission};
use crate::db::user::User;
use crate::utils::update_permission_cache_for_user;

pub fn get_all_permissions(conn: &PgConnection) -> Result<Vec<Permission>, diesel::result::Error> {
    use crate::db::schema::permissions::dsl::*;
    permissions.load(conn)
}

pub fn grant_permissions(
    user: &User,
    new_permissions: Vec<UserPermission>,
    conn: &PgConnection,
) -> Result<(), diesel::result::Error> {
    use crate::db::schema::user_permissions::dsl::*;

    let current_permissions: Vec<i32> = user_permissions
        .select(permission_id)
        .filter(user_id.eq(&user.id))
        .load::<i32>(conn)?;

    let new_permissions = new_permissions.into_iter()
        .filter(|new_permission| {
            !current_permissions.contains(&new_permission.permission_id)
        })
        .map(|permission| permission.into())
        .collect::<Vec<NewUserPermission>>();

    diesel::insert_into(user_permissions)
        .values(&new_permissions)
        .execute(conn)?;

    update_permission_cache_for_user(user, conn)?;

    Ok(())
}

pub fn revoke_permissions(
    user: &User,
    old_permissions: Vec<UserPermission>,
    conn: &PgConnection,
) -> Result<(), diesel::result::Error> {
    use crate::db::schema::user_permissions::dsl::*;

    let old_permissions = old_permissions
        .into_iter()
        .map(|permission| permission.permission_id)
        .collect::<Vec<i32>>();

    diesel::delete(user_permissions
        .filter(
            user_id
                .eq(&user.id)
                .and(permission_id.eq_any(old_permissions))
        )
    )
        .execute(conn)?;

    update_permission_cache_for_user(user, conn)?;

    Ok(())
}

pub fn set_permissions(
    user: &User,
    new_permissions: Vec<UserPermission>,
    conn: &PgConnection,
) -> Result<(), diesel::result::Error> {
    use crate::db::schema::user_permissions::dsl::*;

    diesel::delete(user_permissions
        .filter(
            user_id.eq(&user.id)
        )
    )
        .execute(conn)?;

    let new_permissions = new_permissions
        .into_iter()
        .map(|permission| permission.into())
        .collect::<Vec<NewUserPermission>>();

    diesel::insert_into(user_permissions)
        .values(&new_permissions)
        .execute(conn)?;

    update_permission_cache_for_user(user, conn)?;

    Ok(())
}
