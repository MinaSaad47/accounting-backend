use rocket::serde::{Serialize, Deserialize};
use sqlx::FromRow;

use crate::types::uuid::Uuid;

#[derive(Serialize, Deserialize, FromRow)]
#[serde(crate = "rocket::serde")]
pub struct Funder {
    pub name: String,
    #[serde(default)]
    pub company_id: Uuid,
}
