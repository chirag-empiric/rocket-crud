#[macro_use]
extern crate rocket;
pub mod api;
mod db;

use api::{delete_user, get_all_users, get_user};

#[launch]
async fn rocket() -> _ {
    let db_connection = db::connect()
        .await
        .expect("Error While connecting database");
    rocket::build()
        .mount("/", routes![get_user, get_all_users, delete_user])
        .manage(db_connection)
}
