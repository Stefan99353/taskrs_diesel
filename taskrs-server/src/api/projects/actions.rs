use diesel::pg::Pg;
use diesel::PgConnection;
use diesel::prelude::*;

use diesel_pagination::{LoadPaginated, PaginationPage};

use crate::db::project::{Project, ProjectColumns};
use crate::models::request_filter::{Order, RequestFilter};

pub fn get_all_projects(
    filter: RequestFilter<ProjectColumns>,
    conn: &PgConnection,
) -> Result<PaginationPage<Project>, diesel::result::Error> {
    use crate::db::schema::projects;

    let mut db_query = projects::table.into_boxed::<Pg>();

    // Filter query
    if let Some(query) = filter.query {
        let query = format!("%{}%", query);
        db_query = db_query.filter(projects::name
            .like(query.clone())
            .or(projects::name.like(query))
        );
    }

    // Order by
    let order_by = filter.order_by.unwrap_or(ProjectColumns::Name);
    let order = filter.order.unwrap_or(Order::Ascending);

    db_query = match order {
        Order::Ascending => {
            match order_by {
                ProjectColumns::Id => db_query.order(projects::id.asc()),
                ProjectColumns::Name => db_query.order((projects::name.asc(), projects::id.asc())),
                ProjectColumns::Description => db_query.order((projects::description.asc(), projects::id.asc())),
                ProjectColumns::CategoryId => db_query.order((projects::category_id.asc(), projects::id.asc())),
                ProjectColumns::OwnerId => db_query.order((projects::owner_id.asc(), projects::id.asc())),
                ProjectColumns::CreatorId => db_query.order((projects::creator_id.asc(), projects::id.asc())),
                ProjectColumns::UpdatedAt => db_query.order((projects::updated_at.asc(), projects::id.asc())),
                ProjectColumns::CreatedAt => db_query.order((projects::created_at.asc(), projects::id.asc())),
            }
        }
        Order::Descending => {
            match order_by {
                ProjectColumns::Id => db_query.order(projects::id.desc()),
                ProjectColumns::Name => db_query.order((projects::name.desc(), projects::id.asc())),
                ProjectColumns::Description => db_query.order((projects::description.desc(), projects::id.asc())),
                ProjectColumns::CategoryId => db_query.order((projects::category_id.desc(), projects::id.asc())),
                ProjectColumns::OwnerId => db_query.order((projects::owner_id.desc(), projects::id.asc())),
                ProjectColumns::CreatorId => db_query.order((projects::creator_id.desc(), projects::id.asc())),
                ProjectColumns::UpdatedAt => db_query.order((projects::updated_at.desc(), projects::id.asc())),
                ProjectColumns::CreatedAt => db_query.order((projects::created_at.desc(), projects::id.asc())),
            }
        }
    };

    db_query.load_with_pagination(conn, filter.page, filter.limit)
}

pub fn create_category(category: Category, conn: &PgConnection) -> diesel::QueryResult<CreateEntityResult<Category>> {
    if category.exists(conn)? {
        debug!("Category '{}' already exists in parent", &category.name);
        return Ok(CreateEntityResult::Exists);
    }

    Ok(CreateEntityResult::Ok(category.insert(conn)?))
}

