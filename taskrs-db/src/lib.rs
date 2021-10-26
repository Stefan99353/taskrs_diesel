#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
use diesel::pg::Pg;
use diesel::r2d2::ConnectionManager;
use log::{debug, error, trace};
use r2d2::Pool;

use models::permission::{NewUserPermission, Permission};
use models::user::User;

pub mod models;
pub mod schema;

pub type Db = Pg;
pub type DbConnection = PgConnection;
pub type DbPool = Pool<ConnectionManager<DbConnection>>;

embed_migrations!("migrations");

pub fn connect_database(
    host: &str,
    port: &u16,
    database: &str,
    user: &str,
    password: &str,
) -> Result<DbPool, r2d2::Error> {
    debug!("Connecting to database");
    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        user, password, host, port, database,
    );
    trace!("Database URL: {}", &database_url);

    let connection_manager = ConnectionManager::<PgConnection>::new(&database_url);
    Pool::builder().build(connection_manager).map_err(|e| {
        error!("Could not create database connections: {}", &e);
        e
    })
}

pub fn run_migrations(
    root_email: &str,
    root_password: &str,
    seed_root_permissions: bool,
    conn: &DbConnection,
) -> anyhow::Result<()> {
    embedded_migrations::run(conn)?;

    let mut root_user = User::find_by_email(root_email, conn)?;

    if root_user.is_none() {
        debug!("Seeding root user '{}'", root_email);

        let mut new_root_user = User {
            id: 0,
            email: root_email.to_string(),
            password: root_password.to_string(),
            first_name: None,
            last_name: None,
            activated: true,
            updated_at: None,
            created_at: None,
        };

        new_root_user.hash_password()?;

        root_user = Some(new_root_user.insert(conn)?);
    }

    if let (Some(root_user), true) = (root_user, seed_root_permissions) {
        debug!("Seeding permissions for root user");
        use schema::{permissions, user_permissions, users};

        let root_permissions: Vec<Permission> = permissions::table
            .filter(
                permissions::id.ne_all(
                    user_permissions::table
                        .inner_join(users::table)
                        .filter(users::email.eq(root_email))
                        .select(user_permissions::permission_id),
                ),
            )
            .load::<Permission>(conn)?;

        let new_root_permissions = root_permissions
            .into_iter()
            .map(|per| NewUserPermission {
                user_id: root_user.id,
                permission_id: per.id,
            })
            .collect::<Vec<NewUserPermission>>();

        diesel::insert_into(user_permissions::table)
            .values(new_root_permissions)
            .execute(conn)?;
    }

    Ok(())
}

pub fn update_permissions(
    all_permissions: Vec<&Permission>,
    conn: &DbConnection,
) -> anyhow::Result<()> {
    debug!("Inserting permissions");
    use schema::permissions;

    // Get db permissions
    let db_permissions: Vec<Permission> = permissions::table.load(conn)?;

    // Get all names
    let db_permission_names: Vec<String> = db_permissions.iter().map(|x| x.name.clone()).collect();
    let all_permission_names: Vec<String> =
        all_permissions.iter().map(|x| x.name.clone()).collect();

    let mapped_permissions: Vec<(Option<&&Permission>, Option<&Permission>)> =
        vec![db_permission_names, all_permission_names]
            .iter()
            .flatten()
            .map(|name| {
                let permission = all_permissions.iter().find(|x| &x.name == name);
                let db_permission = db_permissions.iter().find(|x| &x.name == name);

                (permission, db_permission)
            })
            .collect();

    for mapped_permissions in mapped_permissions {
        match mapped_permissions {
            (Some(new), Some(old)) => {
                // Update
                if new.name != old.name {
                    diesel::update(permissions::table.find(old.id))
                        .set(permissions::name.eq(&new.name))
                        .execute(conn)?;
                }
                if new.group != old.group {
                    diesel::update(permissions::table.find(old.id))
                        .set(permissions::group.eq(&new.group))
                        .execute(conn)?;
                }
                if new.description != old.description {
                    diesel::update(permissions::table.find(old.id))
                        .set(permissions::description.eq(&new.description))
                        .execute(conn)?;
                }
            }
            (Some(new), None) => {
                // Insert
                (*new).clone().insert(conn)?;
            }
            (None, Some(old)) => {
                // Delete
                old.clone().delete(conn)?;
            }
            (None, None) => unreachable!("How did we get here?"),
        }
    }

    Ok(())
}
