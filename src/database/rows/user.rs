use rocket::serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
#[serde(crate = "rocket::serde")]
pub struct User {
    #[serde(default)]
    pub id: Option<i64>,
    pub name: String,
    pub password: String,
}
