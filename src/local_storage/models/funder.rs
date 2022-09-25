use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub struct Funder {
    pub id: i64,
    pub name: String,
    pub company_id: i64,
}

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub struct CreateFunder {
    pub name: String,
}