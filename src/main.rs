use dotenv;

#[macro_use]
extern crate rocket;
pub mod api;
mod db;

use api::{create_user, delete_user, get_all_users, get_user, update_user};
use rocket::Config;

#[launch]

async fn rocket() -> _ {
    dotenv::dotenv().ok();

    let db = db::connect().await.unwrap().database("sample_mflix");

    let rocket_config = Config::figment();
    rocket::custom(rocket_config)
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
        .manage(db)
}
