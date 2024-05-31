use crate::modal::{User, UserResponse};
use mongodb::bson::oid::ObjectId;
use mongodb::{bson::doc, options::FindOneOptions, Database};
use rocket::State;
use rocket::{futures::TryStreamExt, serde::json::Json};
use std::str::FromStr;

#[get("/user/get/all")]
pub async fn get_all_users(db: &State<Database>) -> Result<Json<Vec<UserResponse>>, String> {
    let mut users = db
        .collection::<UserResponse>("users")
        .find(None, None)
        .await
        .unwrap();

    let count = db
        .collection::<UserResponse>("users")
        .count_documents(None, None)
        .await
        .unwrap();

    let mut user_list: Vec<UserResponse> = vec![];

    for _i in 0..count {
        let user = users.try_next().await.unwrap();
        user_list.push(user.unwrap());
    }

    Ok(Json(user_list))
}

#[get("/user/get/<id>")]
pub async fn get_user(id: String, db: &State<Database>) -> Result<Json<UserResponse>, String> {
    let find_options = FindOneOptions::builder().skip(0).build();

    let user_result = db
        .collection::<UserResponse>("users")
        .find_one(
            doc! {
                "_id": ObjectId::from_str(&id).unwrap()
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
pub async fn create_user(
    user: Json<User>,
    db: &State<Database>,
) -> Result<Json<UserResponse>, String> {
    let doc = User::new(
        user.name.to_string(),
        user.email.to_string(),
        user.password.to_string(),
    );

    let _ = db.collection::<User>("users").insert_one(doc, None).await;

    let created_user = db
        .collection::<UserResponse>("users")
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

#[put("/user/update/<id>", data = "<user>")]
pub async fn update_user(
    id: String,
    user: Json<User>,
    db: &State<Database>,
) -> Result<Json<UserResponse>, String> {
    let filter = doc! {
       "_id" : ObjectId::from_str(&id).unwrap()
    };

    let update = doc! {
        "$set" : doc! {
            "name" : user.name.to_string(),
            "email" : user.email.to_string(),
            "password" : user.password.to_string(),
        }
    };

    let user = db
        .collection::<UserResponse>("users")
        .find_one_and_update(filter, update, None)
        .await
        .unwrap()
        .unwrap();

    Ok(Json(user))
}

#[delete("/user/delete/<id>")]
pub async fn delete_user(id: String, db: &State<Database>) -> Result<Json<UserResponse>, String> {
    let deleted_user = db
        .collection::<UserResponse>("users")
        .find_one(
            doc! {
                "id": ObjectId::from_str(&id).unwrap()
            },
            None,
        )
        .await
        .unwrap();
    let _ = db
        .collection::<UserResponse>("users")
        .delete_one(
            doc! {
                "id" : ObjectId::from_str(&id).unwrap()
            },
            None,
        )
        .await;

    Ok(Json(deleted_user.unwrap()))
}
