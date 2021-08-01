use diesel::PgConnection;
use diesel::prelude::*;

use crate::db::user::User;

use super::controller::CreateUserResult;

pub fn get_all_users(conn: &PgConnection) -> Result<Vec<User>, diesel::result::Error> {
    use crate::db::schema::users::dsl::*;
    users.load(conn)
}

pub fn create_user(user: User, conn: &PgConnection) -> anyhow::Result<CreateUserResult> {
    if user.exists(conn)? {
        debug!("User '{}' already exists", &user.email);
        return Ok(CreateUserResult::Exists);
    }

    let mut user = user;
    user.hash_password()?;

    Ok(CreateUserResult::Ok(user.insert(conn)?))
}