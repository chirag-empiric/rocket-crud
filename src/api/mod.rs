use mongodb::{bson::doc, options::FindOneOptions, Database};
use rocket::State;
use rocket::{futures::TryStreamExt, serde::json::Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    name: String,
    email: String,
    password: String,
}

#[get("/user/get/all")]
pub async fn get_all_users(db: &State<Database>) -> Result<Json<Vec<User>>, String> {
    let mut users = db
        .collection::<User>("users")
        .find(None, None)
        .await
        .unwrap();

    let count = db
        .collection::<User>("users")
        .count_documents(None, None)
        .await
        .unwrap();

    let mut user_list: Vec<User> = vec![];

    for _i in 0..count {
        let user = users.try_next().await.unwrap();
        user_list.push(user.unwrap());
    }

    Ok(Json(user_list))
}

#[get("/user/get/<name>")]
pub async fn get_user(name: String, db: &State<Database>) -> Result<Json<User>, String> {
    let find_options = FindOneOptions::builder().skip(0).build();

    let user_result = db
        .collection::<User>("users")
        .find_one(
            doc! {
                "name": name
            },
            find_options,
        )
        .await;

    match user_result {
        Ok(Some(user)) => Ok(Json(user)),
        Ok(None) => Err("No user found".to_string()),
        Err(e) => Err(format!("Error finding user: {:?}", e)),
    }
}

#[post("/user/new", data = "<user>")]
pub async fn create_user(user: Json<User>, db: &State<Database>) -> Result<Json<User>, String> {
    let doc = User {
        name: user.name.to_string(),
        email: user.email.to_string(),
        password: user.password.to_string(),
    };
    let _ = db.collection::<User>("users").insert_one(doc, None).await;

    let created_user = db
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
    db: &State<Database>,
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

    let user = db
        .collection::<User>("users")
        .find_one_and_update(filter, update, None)
        .await
        .unwrap()
        .unwrap();

    Ok(Json(user))
}

#[delete("/user/delete/<username>")]
pub async fn delete_user(username: String, db: &State<Database>) -> Result<Json<User>, String> {
    let deleted_user = db
        .collection::<User>("users")
        .find_one(
            doc! {
                "name":username.to_string()
            },
            None,
        )
        .await
        .unwrap();
    let _ = db
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
