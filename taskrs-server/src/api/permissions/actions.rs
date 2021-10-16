use diesel::dsl::count;
use diesel::pg::Pg;
use diesel::prelude::*;
use diesel::PgConnection;

use diesel_pagination::{LoadPaginated, PaginationPage};

use crate::api::permissions::ChangePermissionResult;
use crate::db::permission::{NewUserPermission, Permission, PermissionColumns};
use crate::db::user::User;
use crate::models::request_filter::{Order, RequestFilter};
use crate::utils::update_permission_cache_for_user;

use super::UserPermissionsDto;

pub fn get_all_permissions(
    filter: RequestFilter<PermissionColumns>,
    conn: &PgConnection,
) -> Result<PaginationPage<Permission>, diesel::result::Error> {
    use crate::db::schema::permissions;

    let mut db_query = permissions::table.into_boxed::<Pg>();

    // Filter query
    if let Some(query) = filter.query {
        let query = format!("%{}%", query);
        db_query = db_query.filter(
            permissions::name
                .like(query.clone())
                .or(permissions::group.like(query.clone()))
                .or(permissions::description.like(query)),
        );
    }

    // Order by
    let order_by = filter.order_by.unwrap_or(PermissionColumns::Name);
    let order = filter.order.unwrap_or(Order::Ascending);

    db_query = match order {
        Order::Ascending => match order_by {
            PermissionColumns::Id => db_query.order(permissions::id.asc()),
            PermissionColumns::Name => {
                db_query.order((permissions::name.asc(), permissions::id.asc()))
            }
            PermissionColumns::Group => {
                db_query.order((permissions::group.asc(), permissions::id.asc()))
            }
            PermissionColumns::Description => {
                db_query.order((permissions::description.asc(), permissions::id.asc()))
            }
            PermissionColumns::UpdatedAt => {
                db_query.order((permissions::updated_at.asc(), permissions::id.asc()))
            }
            PermissionColumns::CreatedAt => {
                db_query.order((permissions::created_at.asc(), permissions::id.asc()))
            }
        },
        Order::Descending => match order_by {
            PermissionColumns::Id => db_query.order(permissions::id.desc()),
            PermissionColumns::Name => {
                db_query.order((permissions::name.desc(), permissions::id.asc()))
            }
            PermissionColumns::Group => {
                db_query.order((permissions::group.desc(), permissions::id.asc()))
            }
            PermissionColumns::Description => {
                db_query.order((permissions::description.desc(), permissions::id.asc()))
            }
            PermissionColumns::UpdatedAt => {
                db_query.order((permissions::updated_at.desc(), permissions::id.asc()))
            }
            PermissionColumns::CreatedAt => {
                db_query.order((permissions::created_at.desc(), permissions::id.asc()))
            }
        },
    };

    db_query.load_with_pagination(conn, filter.page, filter.limit)
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
    if count != 1 {
        return Ok(ChangePermissionResult::InvalidUser);
    }

    let current_permissions: Vec<i32> = user_permissions::table
        .select(user_permissions::permission_id)
        .filter(user_permissions::user_id.eq(&new_permissions.user_id))
        .load::<i32>(conn)?;

    let new_permissions = new_permissions
        .permission_ids
        .iter()
        .filter(|permission_id| !current_permissions.contains(permission_id))
        .map(|permission_id| NewUserPermission {
            user_id: new_permissions.user_id,
            permission_id: *permission_id,
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
    if count != 1 {
        return Ok(ChangePermissionResult::InvalidUser);
    }

    diesel::delete(
        user_permissions::table.filter(
            user_permissions::user_id
                .eq(&old_permissions.user_id)
                .and(user_permissions::permission_id.eq_any(old_permissions.permission_ids)),
        ),
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
    if count != 1 {
        return Ok(ChangePermissionResult::InvalidUser);
    }

    diesel::delete(
        user_permissions::table.filter(user_permissions::user_id.eq(&new_permissions.user_id)),
    )
    .execute(conn)?;

    let new_permissions = new_permissions
        .permission_ids
        .iter()
        .map(|permission_id| NewUserPermission {
            user_id: new_permissions.user_id,
            permission_id: *permission_id,
        })
        .collect::<Vec<NewUserPermission>>();

    diesel::insert_into(user_permissions::table)
        .values(&new_permissions)
        .execute(conn)?;

    update_permission_cache_for_user(user, conn)?;

    Ok(ChangePermissionResult::Ok)
}
