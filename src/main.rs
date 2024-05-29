#[macro_use]
extern crate rocket;
pub mod api;
mod db;

use api::{get_all_users, register};
use db::connect;

#[launch]
fn rocket() -> _ {
    let connection = connect().unwrap();
    rocket::build()
        .manage(connection)
        .mount("/", routes![get_all_users, register])
}
