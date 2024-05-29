use mongodb::{bson::doc, options::FindOneOptions, Client};
use rocket::State;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct User {
    username: String,
    // email: String,
    password: String,
    // password: String,
}

#[get("/user/all")]
pub async fn get_all_users(connection: &State<Client>) -> String {
    let user = FindOneOptions::builder().skip(0).build();

    let u: Option<_> = connection
        .database("test")
        .collection::<User>("rust")
        .find_one(
            doc! {
                "username" : "chiragjani"
            },
            user,
        )
        .await
        .unwrap();
    println!("User is: {:?}", u);
    String::from("value")
}

#[get("/user/new")]
pub async fn register(connection: &State<Client>) -> String {
    let u = connection
        .database("test")
        .collection("rust")
        .insert_one(
            doc! {
                "username": "New".to_string(),
                "password": "New".to_string()
            },
            None,
        )
        .await
        .unwrap();

    println!("User is: {:?}", u);
    String::from("value")
}
