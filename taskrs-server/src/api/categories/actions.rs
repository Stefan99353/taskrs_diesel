use diesel::pg::Pg;
use diesel::prelude::*;
use diesel::PgConnection;

use diesel_pagination::{LoadPaginated, PaginationPage};

use crate::api::categories::SubCategoryFilter;
use crate::db::category::{Category, CategoryColumns};
use crate::models::create_entity_result::CreateEntityResult;
use crate::models::delete_entity::{DeleteEntityParams, DeleteEntityResult};
use crate::models::request_filter::{Order, RequestFilter};

pub fn get_all_categories(
    filter: RequestFilter<CategoryColumns>,
    conn: &PgConnection,
) -> Result<PaginationPage<Category>, diesel::result::Error> {
    use crate::db::schema::categories;

    let mut db_query = categories::table.into_boxed::<Pg>();

    // Filter query
    if let Some(query) = filter.query {
        let query = format!("%{}%", query);
        db_query = db_query.filter(
            categories::name
                .like(query.clone())
                .or(categories::name.like(query)),
        );
    }

    // Order by
    let order_by = filter.order_by.unwrap_or(CategoryColumns::Name);
    let order = filter.order.unwrap_or(Order::Ascending);

    db_query = match order {
        Order::Ascending => match order_by {
            CategoryColumns::Id => db_query.order(categories::id.asc()),
            CategoryColumns::Name => db_query.order((categories::name.asc(), categories::id.asc())),
            CategoryColumns::ParentCategoryId => {
                db_query.order((categories::parent_category_id.asc(), categories::id.asc()))
            }
            CategoryColumns::UpdatedAt => {
                db_query.order((categories::updated_at.asc(), categories::id.asc()))
            }
            CategoryColumns::CreatedAt => {
                db_query.order((categories::created_at.asc(), categories::id.asc()))
            }
        },
        Order::Descending => match order_by {
            CategoryColumns::Id => db_query.order(categories::id.desc()),
            CategoryColumns::Name => {
                db_query.order((categories::name.desc(), categories::id.asc()))
            }
            CategoryColumns::ParentCategoryId => {
                db_query.order((categories::parent_category_id.desc(), categories::id.asc()))
            }
            CategoryColumns::UpdatedAt => {
                db_query.order((categories::updated_at.desc(), categories::id.asc()))
            }
            CategoryColumns::CreatedAt => {
                db_query.order((categories::created_at.desc(), categories::id.asc()))
            }
        },
    };

    db_query.load_with_pagination(conn, filter.page, filter.limit)
}

pub fn sub_categories(
    filter: SubCategoryFilter,
    conn: &PgConnection,
) -> diesel::QueryResult<Vec<Category>> {
    use crate::db::schema::categories;

    categories::table
        .filter(categories::parent_category_id.eq(filter.id))
        .load(conn)
}

pub fn create_category(
    category: Category,
    conn: &PgConnection,
) -> diesel::QueryResult<CreateEntityResult<Category>> {
    if category.exists(conn)? {
        debug!("Category '{}' already exists in parent", &category.name);
        return Ok(CreateEntityResult::Exists);
    }

    Ok(CreateEntityResult::Ok(category.insert(conn)?))
}

pub fn delete_category(
    params: DeleteEntityParams,
    conn: &PgConnection,
) -> diesel::QueryResult<DeleteEntityResult<Category>> {
    use crate::db::schema::categories;

    conn.transaction::<DeleteEntityResult<Category>, diesel::result::Error, _>(|| {
        let count = if let Some(true) = params.cascade {
            delete_category_with_dependencies(params.id, conn)?
        } else {
            let sub_categories: Vec<Category> = categories::table
                .filter(categories::parent_category_id.eq(params.id))
                .load(conn)?;

            if !sub_categories.is_empty() {
                return Ok(DeleteEntityResult::Referenced(sub_categories));
            }

            diesel::delete(categories::table.filter(categories::id.eq(params.id))).execute(conn)?
        };

        if count > 0 {
            Ok(DeleteEntityResult::Ok)
        } else {
            Ok(DeleteEntityResult::NotFound)
        }
    })
}

pub fn update_category(
    category: Category,
    conn: &PgConnection,
) -> diesel::QueryResult<Option<Category>> {
    use crate::db::schema::categories;

    let target = categories::table.find(category.id);
    diesel::update(target)
        .set((
            categories::name.eq(category.name),
            categories::parent_category_id.eq(category.parent_category_id),
        ))
        .get_result::<Category>(conn)
        .optional()
}

fn delete_category_with_dependencies(
    category_id: i32,
    conn: &PgConnection,
) -> diesel::QueryResult<usize> {
    use crate::db::schema::categories;

    let sub_categories: Vec<Category> = categories::table
        .filter(categories::parent_category_id.eq(category_id))
        .load(conn)?;

    for sub_category in sub_categories {
        delete_category_with_dependencies(sub_category.id, conn)?;
    }

    diesel::delete(categories::table.filter(categories::id.eq(category_id))).execute(conn)
}
