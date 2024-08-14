#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;

mod user;
mod schema;
mod utils;
mod tasks;

use rocket_sync_db_pools::database;

#[database("postgres_db")]
pub struct DbConn(diesel::PgConnection);

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(DbConn::fairing())
        .mount("/api/v1", user::routes::get_routes())
        .mount("/api/v1", tasks::routes::get_routes())
}