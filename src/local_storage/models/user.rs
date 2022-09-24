use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize,  Default)]
#[serde(crate = "rocket::serde", rename = "camelCase")]
pub struct User {
    #[serde(default)]
    pub id: Option<i64>,
    pub name: String,
    pub password: String,
    #[serde(default)]
    pub is_admin: bool,
    #[serde(default)]
    pub value: f64,
}
