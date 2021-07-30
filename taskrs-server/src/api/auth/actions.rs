use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use diesel::prelude::*;

use crate::CONFIG;
use crate::db::auth_refresh_token::AuthRefreshToken;
use crate::db::user::{SimpleUser, User};
use crate::models::user_token::{UserRefreshToken, UserToken};

use super::UserTokensDto;

pub fn login(user: SimpleUser, conn: &PgConnection) -> anyhow::Result<Option<UserTokensDto>> {
    debug!("Find user with email ' {}'", &user.email);
    let db_user = match User::find_by_email(&user.email, conn)? {
        None => return Ok(None),
        Some(user) => user,
    };

    // Check if user is deactivated
    if !db_user.activated {
        debug!("User deactivated");
        return Ok(None);
    }

    let matches = argon2::verify_encoded(&db_user.password, user.password.as_bytes())?;
    if !matches {
        debug!("Wrong password");
        return Ok(None);
    }

    let tokens = generate_tokens(db_user, conn)?;

    Ok(Some(tokens))
}

pub fn logout(refresh_token: String, user: User, conn: &PgConnection) -> anyhow::Result<()> {
    use crate::db::schema::auth_refresh_tokens::dsl::*;

    diesel::delete(auth_refresh_tokens
        .filter(token.eq(&refresh_token))
        .filter(user_id.eq(user.id))
    )
        .execute(conn)?;

    Ok(())
}

pub fn refresh_token(refresh_token: &str, conn: &PgConnection) -> anyhow::Result<Option<String>> {
    debug!("Find refresh token in database: {}", refresh_token);
    let db_refresh_token = match AuthRefreshToken::find(refresh_token, conn)? {
        None => {
            debug!("Refresh token does not exist -> invalid");
            return Ok(None);
        }
        Some(token) => token,
    };

    // Decode Token
    let user_email = jsonwebtoken::decode::<UserRefreshToken>(
        &db_refresh_token.token,
        &jsonwebtoken::DecodingKey::from_secret(CONFIG.refresh_token_secret.as_bytes()),
        &jsonwebtoken::Validation::default(),
    ).ok();

    // Check if token is valid and return None if not
    let user_email = match user_email {
        None => {
            debug!("Refresh token is invalid");
            return Ok(None);
        }
        Some(data) => data.claims.user_email,
    };

    // Get user from database
    debug!("Find user with email ' {}'", &user_email);
    let db_user = match User::find_by_email(&user_email, conn)? {
        None => return Ok(None),
        Some(user) => user,
    };

    debug!("Create new token");
    // TODO: Don't recreate EncodingKey everytime
    let user_token_claim: UserToken = db_user.into();
    let access_key = jsonwebtoken::EncodingKey::from_secret(CONFIG.access_token_secret.as_bytes());
    let access_token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &user_token_claim,
        &access_key,
    )?;

    Ok(Some(access_token))
}

fn generate_tokens(user: User, conn: &PgConnection) -> anyhow::Result<UserTokensDto> {
    let user_id = user.id;
    // TODO: Don't recreate EncodingKey everytime
    let access_key = jsonwebtoken::EncodingKey::from_secret(CONFIG.access_token_secret.as_bytes());
    let claim: UserToken = user.clone().into();
    let access_token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claim,
        &access_key,
    )?;

    // TODO: Don't recreate EncodingKey everytime
    let refresh_key = jsonwebtoken::EncodingKey::from_secret(CONFIG.refresh_token_secret.as_bytes());
    let refresh_claim: UserRefreshToken = user.into();
    let refresh_token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &refresh_claim,
        &refresh_key,
    )?;

    // Save refresh token in DB
    AuthRefreshToken {
        id: 0,
        user_id,
        token: refresh_token.clone(),
        iat: refresh_claim.iat,
        exp: refresh_claim.exp,
        updated_at: None,
        created_at: None,
    }.insert(conn)?;

    Ok(UserTokensDto {
        access_token,
        refresh_token,
    })
}

pub fn revoke_token(
    refresh_token: &str,
    conn: &PgConnection,
) -> anyhow::Result<()> {
    use crate::db::schema::auth_refresh_tokens::dsl::*;
    debug!("Delete refresh token in database: {}", refresh_token);

    diesel::delete(auth_refresh_tokens.filter(token.eq(refresh_token)))
        .execute(conn)?;

    Ok(())
}