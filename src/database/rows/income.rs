use chrono::{DateTime, Utc};
use rocket::serde::{Deserialize, Serialize};
use sqlx::FromRow;


#[derive(Serialize, Deserialize, FromRow, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Income {
    #[serde(default)]
    pub id: Option<i64>,
    pub value: f64,
    pub description: String,
    #[serde(default)]
    pub time: Option<DateTime<Utc>>,
    #[serde(default)]
    pub company_id: i64,
    #[serde(default)]
    pub admin_id: i64,
}
