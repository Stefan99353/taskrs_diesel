use chrono::NaiveDateTime;
use diesel::{Insertable, QueryDsl, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::schema::{permissions, user_permissions};
use crate::DbConnection;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PermissionColumns {
    Id,
    Name,
    Group,
    Description,
    UpdatedAt,
    CreatedAt,
}

#[derive(
    Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Default, Serialize, Deserialize, Queryable,
)]
#[serde(rename_all = "camelCase")]
pub struct Permission {
    pub id: i32,
    pub name: String,
    pub group: String,
    pub description: Option<String>,
    pub updated_at: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
}

impl Permission {
    pub fn insert(self, conn: &DbConnection) -> diesel::QueryResult<Permission> {
        let new_permission: NewPermission = self.into();
        diesel::insert_into(permissions::table)
            .values(new_permission)
            .get_result(conn)
    }

    pub fn insert_many(
        values: &[Permission],
        conn: &DbConnection,
    ) -> diesel::QueryResult<Vec<Permission>> {
        let new_permissions: Vec<NewPermission> =
            values.iter().cloned().map(|per| per.into()).collect();

        diesel::insert_into(permissions::table)
            .values(new_permissions)
            .get_results(conn)
    }

    pub fn delete(self, conn: &DbConnection) -> diesel::QueryResult<usize> {
        diesel::delete(permissions::table.find(self.id)).execute(conn)
    }
}

#[derive(Debug, Clone, Insertable)]
#[table_name = "permissions"]
struct NewPermission {
    pub name: String,
    pub group: String,
    pub description: Option<String>,
}

impl From<Permission> for NewPermission {
    fn from(
        Permission {
            name,
            group,
            description,
            ..
        }: Permission,
    ) -> Self {
        Self {
            name,
            group,
            description,
        }
    }
}

#[derive(
    Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Default, Serialize, Deserialize, Queryable,
)]
#[serde(rename_all = "camelCase")]
pub struct UserPermission {
    pub user_id: i32,
    pub permission_id: i32,
    pub updated_at: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
}

impl UserPermission {
    pub fn insert(
        user_id: i32,
        permission_id: i32,
        conn: &DbConnection,
    ) -> diesel::QueryResult<UserPermission> {
        let new_user_permission = NewUserPermission {
            user_id,
            permission_id,
        };
        diesel::insert_into(user_permissions::table)
            .values(new_user_permission)
            .get_result(conn)
    }
}

#[derive(Debug, Clone, Insertable)]
#[table_name = "user_permissions"]
pub struct NewUserPermission {
    pub user_id: i32,
    pub permission_id: i32,
}

impl From<UserPermission> for NewUserPermission {
    fn from(
        UserPermission {
            user_id,
            permission_id,
            ..
        }: UserPermission,
    ) -> Self {
        Self {
            user_id,
            permission_id,
        }
    }
}
