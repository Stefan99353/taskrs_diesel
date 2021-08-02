use actix_web::{Error, FromRequest, HttpRequest, HttpResponse};
use actix_web::dev::Payload;
use chrono::NaiveDateTime;
use diesel::{Insertable, PgConnection, Queryable};
use diesel::prelude::*;
use futures::future::{err, ok, Ready};
use serde::{Deserialize, Serialize};

use super::schema::users;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum UserColumns {
    Id,
    Email,
    Password,
    FirstName,
    LastName,
    Activated,
    UpdatedAt,
    CreatedAt,
}

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

impl FromRequest for User {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let mut user: Option<User> = None;

        // Get Authorization Header
        if let Some(auth_header) = req.headers().get("authorization") {
            if let Ok(auth_str) = auth_header.to_str() {
                // Check if Bearer Token
                if auth_str.starts_with("bearer") || auth_str.starts_with("Bearer") {
                    // Trim Bearer word
                    let token: &str = auth_str[6..auth_str.len()].trim();
                    // Decode token
                    if let Ok(token_data) = crate::utils::decode_token(token) {
                        user = Some(token_data.claims.user);
                    }
                }
            }
        }

        if let Some(user) = user {
            ok(user)
        } else {
            err(HttpResponse::Unauthorized().finish().into())
        }
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
