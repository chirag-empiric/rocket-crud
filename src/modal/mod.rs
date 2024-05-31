use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub name: String,
    pub email: String,
    pub password: String,
}

impl User {
    pub fn new(name: String, email: String, password: String) -> Self {
        User {
            name,
            email,
            password,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserResponse {
    pub _id: ObjectId,
    pub name: String,
    pub email: String,
    pub password: String,
}
