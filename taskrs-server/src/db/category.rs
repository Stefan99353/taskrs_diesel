use chrono::NaiveDateTime;
use diesel::{Insertable, PgConnection, Queryable};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use super::schema::categories;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CategoryColumns {
    Id,
    Name,
    ParentCategoryId,
    UpdatedAt,
    CreatedAt,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Queryable)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub parent_category_id: Option<i32>,
    pub updated_at: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
}

impl Category {
    pub fn insert(self, conn: &PgConnection) -> diesel::QueryResult<Category> {
        use crate::db::schema::categories::dsl::*;

        let new_category: NewCategory = self.into();
        diesel::insert_into(categories)
            .values(new_category)
            .get_result(conn)
    }
}

#[derive(Debug, Clone, Insertable)]
#[table_name = "categories"]
struct NewCategory {
    pub name: String,
    pub parent_category_id: Option<i32>,
}

impl From<Category> for NewCategory {
    fn from(Category { name, parent_category_id, .. }: Category) -> Self {
        Self {
            name,
            parent_category_id,
        }
    }
}
