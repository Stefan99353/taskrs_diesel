use chrono::NaiveDateTime;
use diesel::{Insertable, PgConnection, Queryable};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use super::schema::users;

#[derive(Debug, Clone, Default, Serialize, Deserialize, Queryable)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    #[serde(default = "default_bool")]
    pub activated: bool,
    pub updated_at: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
}


impl User {
    pub fn insert(self, conn: &PgConnection) -> diesel::QueryResult<User> {
        use crate::db::schema::users::dsl::*;

        let new_user: NewUser = self.into();
        diesel::insert_into(users)
            .values(new_user)
            .get_result(conn)
    }

    pub fn find_by_email(q: &str, conn: &PgConnection) -> diesel::QueryResult<Option<Self>> {
        use crate::db::schema::users::dsl::*;

        users
            .filter(email.eq(q))
            .first::<Self>(conn)
            .optional()
    }

    pub fn exists(&self, conn: &PgConnection) -> diesel::QueryResult<bool> {
        use crate::db::schema::users::dsl::*;

        users.filter(email.eq(&self.email))
            .first::<Self>(conn)
            .optional()
            .map(|user| user.is_some())
    }

    pub fn hash_password(&mut self) -> argon2::Result<()> {
        let salt = rand::random::<[u8; 16]>();
        let config = argon2::Config::default();
        self.password = argon2::hash_encoded(self.password.as_bytes(), &salt, &config)?;

        Ok(())
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Queryable)]
#[serde(rename_all = "camelCase")]
pub struct SimpleUser {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Insertable)]
#[table_name = "users"]
struct NewUser {
    pub email: String,
    pub password: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub activated: bool,
}

impl From<User> for NewUser {
    fn from(User { email, password, first_name, last_name, activated, .. }: User) -> Self {
        Self {
            email,
            password,
            first_name,
            last_name,
            activated,
        }
    }
}

fn default_bool() -> bool { true }
