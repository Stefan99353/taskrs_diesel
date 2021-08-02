#[macro_use]
extern crate diesel;

use diesel::pg::Pg;
use diesel::prelude::*;
use diesel::query_builder::{AstPass, Query, QueryFragment, QueryId};
use diesel::query_dsl::LoadQuery;
use diesel::sql_types::{HasSqlType, Integer};
use serde::{Deserialize, Serialize};

const DEFAULT_PAGE_SIZE: i32 = 25;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginationPage<T> {
    pub page: Option<i32>,
    pub page_count: Option<i32>,
    pub page_size: Option<i32>,
    pub total_count: Option<i32>,
    pub items: Vec<T>,
}

#[derive(QueryId)]
pub struct Paginated<T> {
    query: T,
    page: i32,
    page_size: i32,
}

pub trait Paginate: Sized {
    fn paginate(self, page: i32) -> Paginated<Self>;
}

impl<T> Paginate for T {
    fn paginate(self, page: i32) -> Paginated<Self> {
        Paginated {
            query: self,
            page,
            page_size: DEFAULT_PAGE_SIZE,
        }
    }
}

impl<T> QueryFragment<Pg> for Paginated<T>
    where T: QueryFragment<Pg>,
{
    fn walk_ast(&self, mut pass: AstPass<Pg>) -> QueryResult<()> {
        pass.push_sql("SELECT *, COUNT(*) OVER () FROM (");
        self.query.walk_ast(pass.reborrow())?;
        pass.push_sql(") t LIMIT ");
        pass.push_bind_param::<Integer, _>(&self.page_size)?;
        pass.push_sql(" OFFSET ");
        let offset = self.page * self.page_size;
        pass.push_bind_param::<Integer, _>(&offset)?;

        Ok(())
    }
}

impl<T: Query> Query for Paginated<T> {
    type SqlType = (T::SqlType, Integer);
}

impl<T> RunQueryDsl<PgConnection> for Paginated<T> {}

impl<T> Paginated<T> {
    pub fn page_size(self, page_size: i32) -> Self {
        Paginated { page_size, ..self }
    }

    pub fn load_and_count<U>(self, conn: &PgConnection) -> QueryResult<(Vec<U>, i32)>
        where
            Self: LoadQuery<PgConnection, (U, i32)>,
    {
        let results = self.load::<(U, i32)>(conn)?;
        let total = results.get(0).map(|x| x.1).unwrap_or(0);
        let records = results.into_iter().map(|x| x.0).collect();

        Ok((records, total))
    }
}

pub trait LoadPaginated<U>: Query + QueryId + QueryFragment<Pg> + LoadQuery<PgConnection, U> {
    fn load_with_pagination(self, conn: &PgConnection, page: Option<i32>, page_size: Option<i32>) -> QueryResult<PaginationPage<U>>;
}

impl<T, U> LoadPaginated<U> for T
    where
        Self: Query + QueryId + QueryFragment<Pg> + LoadQuery<PgConnection, U>,
        U: Queryable<Self::SqlType, Pg>,
        Pg: HasSqlType<Self::SqlType>,
{
    fn load_with_pagination(
        self,
        conn: &PgConnection,
        page: Option<i32>,
        page_size: Option<i32>,
    ) -> QueryResult<PaginationPage<U>> {
        let mut result_page = PaginationPage {
            page,
            page_count: None,
            page_size: None,
            total_count: None,
            items: vec![],
        };

        match page {
            None => result_page.items = self.load::<U>(conn)?,
            Some(page) => {
                let mut query = self.paginate(page);
                let page_size = page_size.unwrap_or(DEFAULT_PAGE_SIZE);

                query = query.page_size(page_size);
                result_page.page_size = Some(page_size);

                let (items, total) = query.load_and_count::<U>(conn)?;
                result_page.items = items;
                result_page.total_count = Some(total);
                result_page.page_count = Some((total as f64 / page_size as f64).ceil() as i32);
            }
        }

        Ok(result_page)
    }
}