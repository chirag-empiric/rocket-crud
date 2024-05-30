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

#[get("/user/all")]
pub async fn get_all_users(db_connection: &State<Client>) -> Result<String, String> {
    let users = db_connection
        .inner()
        .database("sample_mflix")
        .collection::<User>("users")
        .find(None, None)
        .await
        .unwrap();

    println!("Found users: {:?}", users);

    Ok("All users fetched, response issues".to_string())
}

#[get("/user/get/<name>")]
pub async fn get_user(name: String, db_connection: &State<Client>) -> Result<Json<User>, String> {
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
            Ok(Json(user))
        }
        Ok(None) => {
            println!("No user found with the given criteria.");
            Err("No user found".to_string())
        }
        Err(e) => {
            eprintln!("Error finding user: {:?}", e);
            Err(format!("Error finding user: {:?}", e))
        }
    }
}

#[post("/user/new", data = "<user>")]
pub async fn create_user(
    user: Json<User>,
    db_connection: &State<Client>,
) -> Result<Json<User>, String> {
    let doc = User {
        name: user.name.to_string(),
        email: user.email.to_string(),
        password: user.password.to_string(),
    };
    let _ = db_connection
        .inner()
        .database("sample_mflix")
        .collection::<User>("users")
        .insert_one(doc, None)
        .await;

    let created_user = db_connection
        .inner()
        .database("sample_mflix")
        .collection::<User>("users")
        .find_one(
            doc! {
                "name":user.name.to_string()
            },
            None,
        )
        .await
        .unwrap();

    Ok(Json(created_user.unwrap()))
}

#[put("/user/update/<name>", data = "<user>")]
pub async fn update_user(
    name: String,
    user: Json<User>,
    db_connection: &State<Client>,
) -> Result<Json<User>, String> {
    let filter = doc! {
       "name" : name.to_string()
    };

    let update = doc! {
        "$set" : doc! {
            "name" : user.name.to_string(),
            "email" : user.email.to_string(),
            "password" : user.password.to_string(),
        }
    };

    let user = db_connection
        .inner()
        .database("sample_mflix")
        .collection::<User>("users")
        .find_one_and_update(filter, update, None)
        .await
        .unwrap()
        .unwrap();

    Ok(Json(user))
}

#[delete("/user/delete/<username>")]
pub async fn delete_user(
    username: String,
    db_connection: &State<Client>,
) -> Result<Json<User>, String> {
    let deleted_user = db_connection
        .inner()
        .database("sample_mflix")
        .collection::<User>("users")
        .find_one(
            doc! {
                "name":username.to_string()
            },
            None,
        )
        .await
        .unwrap();
    let _ = db_connection
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

    Ok(Json(deleted_user.unwrap()))
}
