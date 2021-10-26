use chrono::NaiveDateTime;
use diesel::{
    BoolExpressionMethods, ExpressionMethods, Insertable, OptionalExtension, QueryDsl, Queryable,
    RunQueryDsl,
};
use serde::{Deserialize, Serialize};

use crate::schema::categories;
use crate::DbConnection;

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
    pub fn insert(self, conn: &DbConnection) -> diesel::QueryResult<Category> {
        let new_category: NewCategory = self.into();
        diesel::insert_into(categories::table)
            .values(new_category)
            .get_result(conn)
    }

    pub fn exists(&self, conn: &DbConnection) -> diesel::QueryResult<bool> {
        categories::table
            .filter(
                categories::name
                    .eq(&self.name)
                    .and(categories::parent_category_id.eq(&self.parent_category_id)),
            )
            .first::<Self>(conn)
            .optional()
            .map(|category| category.is_some())
    }
}

#[derive(Debug, Clone, Insertable)]
#[table_name = "categories"]
struct NewCategory {
    pub name: String,
    pub parent_category_id: Option<i32>,
}

impl From<Category> for NewCategory {
    fn from(
        Category {
            name,
            parent_category_id,
            ..
        }: Category,
    ) -> Self {
        Self {
            name,
            parent_category_id,
        }
    }
}
