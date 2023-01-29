use rocket::serde::{Deserialize, Serialize};
use sqlx::types::Uuid;

#[derive(Serialize, Debug)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub struct Funder {
    pub id: Uuid,
    pub name: String,
}

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub struct CreateFunder {
    pub name: String,
}
