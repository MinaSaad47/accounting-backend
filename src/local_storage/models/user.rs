use rocket::serde::{Deserialize, Serialize};
use sqlx::types::Uuid;

#[derive(Serialize, Debug)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub password: String,
    pub is_admin: bool,
    pub value: f64,
}

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub struct RegisterUser {
    pub name: String,
    pub password: String,
    pub is_admin: bool,
}

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub struct LoginUser {
    pub name: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub struct UpdateUser {
    pub name: String,
    pub password: String,
    pub is_admin: bool,
}
