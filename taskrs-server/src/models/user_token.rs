use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::CONFIG;
use crate::db::user::User;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserToken {
    pub iat: i64,
    pub exp: i64,
    pub user: User,
}

impl From<User> for UserToken {
    fn from(user: User) -> Self {
        let now = Utc::now().timestamp();
        UserToken {
            iat: now,
            exp: now + (CONFIG.access_token_expiration_time as i64),
            user,
        }
    }
}

impl From<&User> for UserToken {
    fn from(user: &User) -> Self {
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