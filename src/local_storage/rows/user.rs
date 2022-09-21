use rocket::serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Default)]
#[serde(crate = "rocket::serde")]
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
