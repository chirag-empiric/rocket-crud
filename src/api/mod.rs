use mongodb::{bson::doc, options::FindOneOptions, Client};
use rocket::serde::json::Json;
use rocket::State;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    name: String,
    email: String,
    password: String,
}

#[get("/user/get/<name>")]
pub async fn get_user(name: String, db_connection: &State<Client>) -> Result<String, String> {
    let find_options = FindOneOptions::builder().skip(0).build();

    let user_result = db_connection
        .inner()
        .database("sample_mflix")
        .collection::<User>("users")
        .find_one(
            doc! {
                "name": name
            },
            find_options,
        )
        .await;

    match user_result {
        Ok(Some(user)) => {
            println!("User is: {:?}", user);
            Ok(format!("User found: {:?}", user))
        }
        Ok(None) => {
            println!("No user found with the given criteria.");
            Ok("No user found".to_string())
        }
        Err(e) => {
            eprintln!("Error finding user: {:?}", e);
            Err(format!("Error finding user: {:?}", e))
        }
    }
}

#[post("/user/new", data = "<user>")]
pub async fn get_all_users(
    user: Json<User>,
    db_connection: &State<Client>,
) -> Result<String, String> {
    let doc = User {
        name: user.name.to_string(),
        email: user.email.to_string(),
        password: user.password.to_string(),
    };
    let res = db_connection
        .inner()
        .database("sample_mflix")
        .collection::<User>("users")
        .insert_one(doc, None)
        .await;
    println!("OKKK: {:?}", res);

    Ok("OKK".to_string())
}

#[delete("/user/delete/<username>")]
pub async fn delete_user(
    username: String,
    db_connection: &State<Client>,
) -> Result<String, String> {
    let res = db_connection
        .inner()
        .database("sample_mflix")
        .collection::<User>("users")
        .delete_one(
            doc! {
                "name" : username
            },
            None,
        )
        .await;
    println!("Deleted: {:?}", res);

    Ok("OKK Deleted".to_string())
}
