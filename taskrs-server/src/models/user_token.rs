use actix_web::dev::Payload;
use actix_web::FromRequest;
use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

use taskrs_db::models::user::User;

use crate::CONFIG;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenUser {
    pub id: i32,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub activated: bool,
    pub updated_at: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
}

impl From<User> for TokenUser {
    fn from(
        User {
            id,
            email,
            first_name,
            last_name,
            activated,
            updated_at,
            created_at,
            ..
        }: User,
    ) -> Self {
        TokenUser {
            id,
            email,
            first_name,
            last_name,
            activated,
            updated_at,
            created_at,
        }
    }
}

impl FromRequest for TokenUser {
    type Error = actix_web::Error;
    type Future = futures::future::Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &actix_web::HttpRequest, _payload: &mut Payload) -> Self::Future {
        let mut user: Option<TokenUser> = None;

        // Get Authorization Header
        if let Some(auth_header) = req.headers().get("authorization") {
            if let Ok(auth_str) = auth_header.to_str() {
                // Check if Bearer Token
                if auth_str.starts_with("bearer") || auth_str.starts_with("Bearer") {
                    // Trim Bearer word
                    let token: &str = auth_str[6..auth_str.len()].trim();
                    // Decode token
                    if let Ok(token_data) =
                        crate::utils::decode_token(token, &CONFIG.access_token_secret)
                    {
                        user = Some(token_data.claims.user);
                    }
                }
            }
        }

        if let Some(user) = user {
            futures::future::ok(user)
        } else {
            futures::future::err(actix_web::HttpResponse::Unauthorized().finish().into())
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserToken {
    pub iat: i64,
    pub exp: i64,
    pub user: TokenUser,
}

impl From<TokenUser> for UserToken {
    fn from(user: TokenUser) -> Self {
        let now = Utc::now().timestamp();
        UserToken {
            iat: now,
            exp: now + (CONFIG.access_token_expiration_time as i64),
            user,
        }
    }
}

impl From<&TokenUser> for UserToken {
    fn from(user: &TokenUser) -> Self {
        let now = Utc::now().timestamp();
        UserToken {
            iat: now,
            exp: now + (CONFIG.access_token_expiration_time as i64),
            user: user.clone(),
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserRefreshToken {
    pub iat: i64,
    pub exp: i64,
    pub user_email: String,
}

impl From<TokenUser> for UserRefreshToken {
    fn from(user: TokenUser) -> Self {
        let now = Utc::now().timestamp();
        UserRefreshToken {
            iat: now,
            exp: now + (CONFIG.refresh_token_expiration_time as i64),
            user_email: user.email,
        }
    }
}

impl From<&TokenUser> for UserRefreshToken {
    fn from(user: &TokenUser) -> Self {
        let now = Utc::now().timestamp();
        UserRefreshToken {
            iat: now,
            exp: now + (CONFIG.refresh_token_expiration_time as i64),
            user_email: user.email.clone(),
        }
    }
}

impl From<User> for UserRefreshToken {
    fn from(user: User) -> Self {
        let now = Utc::now().timestamp();
        UserRefreshToken {
            iat: now,
            exp: now + (CONFIG.refresh_token_expiration_time as i64),
            user_email: user.email,
        }
    }
}

impl From<&User> for UserRefreshToken {
    fn from(user: &User) -> Self {
        let now = Utc::now().timestamp();
        UserRefreshToken {
            iat: now,
            exp: now + (CONFIG.refresh_token_expiration_time as i64),
            user_email: user.email.clone(),
        }
    }
}
