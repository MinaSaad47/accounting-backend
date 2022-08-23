use chrono::{DateTime, Utc};
use rocket::serde::{Deserialize, Serialize};
use sqlx::FromRow;


#[derive(Serialize, Deserialize, FromRow, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct MoneyCapital {
    #[serde(default)]
    pub id: Option<i64>,
    pub value: f64,
    #[serde(default)]
    pub time: Option<DateTime<Utc>>,
    #[serde(default)]
    pub company_id: i64,
    #[serde(default)]
    pub user_id: i64,
}
