use chrono::{DateTime, Utc};
use rocket::serde::{Deserialize, Serialize};
use sqlx::types::Uuid;

#[derive(Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Expense {
    pub id: Uuid,
    pub value: f64,
    pub description: String,
    pub time: DateTime<Utc>,
    pub company: String,
    pub user: String,
}

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub struct CreateExpense {
    pub value: f64,
    pub description: String,
}
