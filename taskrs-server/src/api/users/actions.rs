use diesel::pg::Pg;
use diesel::PgConnection;
use diesel::prelude::*;

use diesel_pagination::{LoadPaginated, PaginationPage};

use crate::db::user::{User, UserColumns};
use crate::models::create_entity_result::CreateEntityResult;
use crate::models::request_filter::{Order, RequestFilter};
use crate::models::delete_entity::{DeleteEntityParams, DeleteEntityResult};

pub fn get_all_users(
    filter: RequestFilter<UserColumns>,
    conn: &PgConnection,
) -> Result<PaginationPage<User>, diesel::result::Error> {
    use crate::db::schema::users;

    let mut db_query = users::table.into_boxed::<Pg>();

    // Filter query
    if let Some(query) = filter.query {
        let query = format!("%{}%", query);
        db_query = db_query.filter(users::email
            .like(query.clone())
            .or(users::first_name.like(query.clone()))
            .or(users::last_name.like(query))
        );
    }

    // Order by
    let order_by = filter.order_by.unwrap_or(UserColumns::Email);
    let order = filter.order.unwrap_or(Order::Ascending);

    db_query = match order {
        Order::Ascending => {
            match order_by {
                UserColumns::Id => db_query.order(users::id.asc()),
                UserColumns::Email => db_query.order((users::email.asc(), users::id.asc())),
                UserColumns::Password => db_query.order((users::password.asc(), users::id.asc())),
                UserColumns::FirstName => db_query.order((users::first_name.asc(), users::id.asc())),
                UserColumns::LastName => db_query.order((users::last_name.asc(), users::id.asc())),
                UserColumns::Activated => db_query.order((users::activated.asc(), users::id.asc())),
                UserColumns::UpdatedAt => db_query.order((users::updated_at.asc(), users::id.asc())),
                UserColumns::CreatedAt => db_query.order((users::created_at.asc(), users::id.asc())),
            }
        }
        Order::Descending => {
            match order_by {
                UserColumns::Id => db_query.order(users::id.desc()),
                UserColumns::Email => db_query.order((users::email.desc(), users::id.asc())),
                UserColumns::Password => db_query.order((users::password.desc(), users::id.asc())),
                UserColumns::FirstName => db_query.order((users::first_name.desc(), users::id.asc())),
                UserColumns::LastName => db_query.order((users::last_name.desc(), users::id.asc())),
                UserColumns::Activated => db_query.order((users::activated.desc(), users::id.asc())),
                UserColumns::UpdatedAt => db_query.order((users::updated_at.desc(), users::id.asc())),
                UserColumns::CreatedAt => db_query.order((users::created_at.desc(), users::id.asc())),
            }
        }
    };

    db_query.load_with_pagination(conn, filter.page, filter.limit)
}

pub fn create_user(user: User, conn: &PgConnection) -> anyhow::Result<CreateEntityResult<User>> {
    if user.exists(conn)? {
        debug!("User '{}' already exists", &user.email);
        return Ok(CreateEntityResult::Exists);
    }

    let mut user = user;
    user.hash_password()?;

    Ok(CreateEntityResult::Ok(user.insert(conn)?))
}

pub fn delete_user(
    params: DeleteEntityParams,
    conn: &PgConnection,
) -> diesel::QueryResult<DeleteEntityResult<User>> {
    use crate::db::schema::users;

    let count = diesel::delete(users::table.filter(users::id.eq(params.id)))
        .execute(conn)?;

    if count > 0 {
        Ok(DeleteEntityResult::Ok)
    } else {
        Ok(DeleteEntityResult::NotFound)
    }
}

pub fn update_user(
    user: User,
    conn: &PgConnection,
) -> anyhow::Result<Option<User>> {
    use crate::db::schema::users;

    // hash password
    let mut user = user;
    user.hash_password()?;

    let target = users::table.find(user.id);
    let user = diesel::update(target)
        .set((
            users::email.eq(user.email),
            users::password.eq(user.password),
            users::first_name.eq(user.first_name),
            users::last_name.eq(user.last_name),
            users::activated.eq(user.activated),
        ))
        .get_result::<User>(conn)
        .optional()?;

    Ok(user)
}