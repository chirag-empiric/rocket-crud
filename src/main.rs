#[macro_use]
extern crate rocket;
pub mod api;
mod db;

use api::{create_user, delete_user, get_all_users, get_user, update_user};

#[launch]
async fn rocket() -> _ {
    let db_connection = db::connect()
        .await
        .expect("Error While connecting database");
    rocket::build()
        .mount(
            "/",
            routes![
                get_user,
                create_user,
                update_user,
                delete_user,
                get_all_users
            ],
        )
        .manage(db_connection)
}
