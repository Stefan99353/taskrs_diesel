use chrono::NaiveDateTime;
use diesel::{ExpressionMethods, Insertable, OptionalExtension, QueryDsl, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::schema::auth_refresh_tokens;
use crate::DbConnection;

#[derive(Debug, Clone, Default, Serialize, Deserialize, Queryable)]
#[serde(rename_all = "camelCase")]
pub struct AuthRefreshToken {
    pub id: i32,
    pub user_id: i32,
    pub token: String,
    pub iat: i64,
    pub exp: i64,
    pub updated_at: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
}

impl AuthRefreshToken {
    pub fn insert(self, conn: &DbConnection) -> diesel::QueryResult<AuthRefreshToken> {
        let new_auth_refresh_token: NewAuthRefreshToken = self.into();
        diesel::insert_into(auth_refresh_tokens::table)
            .values(new_auth_refresh_token)
            .get_result(conn)
    }

    pub fn find(q: &str, conn: &DbConnection) -> diesel::QueryResult<Option<Self>> {
        auth_refresh_tokens::table
            .filter(auth_refresh_tokens::token.eq(q))
            .first::<Self>(conn)
            .optional()
    }
}

#[derive(Debug, Clone, Insertable)]
#[table_name = "auth_refresh_tokens"]
struct NewAuthRefreshToken {
    pub user_id: i32,
    pub token: String,
    pub iat: i64,
    pub exp: i64,
}

impl From<AuthRefreshToken> for NewAuthRefreshToken {
    fn from(
        AuthRefreshToken {
            user_id,
            token,
            iat,
            exp,
            ..
        }: AuthRefreshToken,
    ) -> Self {
        Self {
            user_id,
            token,
            iat,
            exp,
        }
    }
}
