use rocket::serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Clone, Serialize, Deserialize, FromRow, Debug, Default)]
#[serde(crate = "rocket::serde")]
pub struct Funder {
    #[serde(default)]
    pub id: Option<i64>,
    pub name: String,
    #[serde(default)]
    pub company_id: i64,
}
