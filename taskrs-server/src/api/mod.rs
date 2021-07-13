pub mod users;
pub mod auth;

use std::sync::Arc;
use crate::db::DbPool;
use r2d2::PooledConnection;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;

// lazy_static! {
//     static ref PERMISSION_CACHE: RwLock<Vec<(i32, i32)>> = RwLock::new(vec![]);
// }

pub fn get_db_connection(pool: Arc<DbPool>) -> Result<PooledConnection<ConnectionManager<PgConnection>>, actix_web::Error> {
    pool.get().map_err(|err| {
        error!("{}", err);
        actix_web::HttpResponse::InternalServerError().finish().into()
    })
}