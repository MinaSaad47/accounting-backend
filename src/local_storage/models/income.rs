use chrono::{DateTime, Utc};
use rocket::serde::{Deserialize, Serialize};
use sqlx::types::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Income {
    #[serde(default)]
    pub id: Uuid,
    pub value: f64,
    pub description: String,
    pub time: DateTime<Utc>,
    pub company: String,
    pub admin: String,
}

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub struct CreateIncome {
    pub value: f64,
    pub description: String,
}
