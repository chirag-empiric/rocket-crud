use mongodb::{bson::doc, options::FindOneOptions, Client};
use rocket::State;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct User {
    name: String,
    email: String,
    password: String,
}

#[derive(Debug, Serialize)]
struct _UserModal {
    name: String,
}

#[get("/user/<name>")]
pub async fn get_user(name: &str, db_connection: &State<Client>) -> Result<String, String> {
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
