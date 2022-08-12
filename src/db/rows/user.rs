use rocket::serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::types::uuid::Uuid;

#[derive(Serialize, Deserialize, FromRow)]
#[serde(crate = "rocket::serde")]
pub struct User {
    #[serde(default)]
    pub id: Uuid,
    pub name: String,
    pub password: String,
}
