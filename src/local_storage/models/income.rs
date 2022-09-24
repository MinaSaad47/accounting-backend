use chrono::{DateTime, Utc};
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize,  Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Income {
    #[serde(default)]
    pub id: Option<i64>,
    pub value: f64,
    pub description: String,
    #[serde(default)]
    pub time: Option<DateTime<Utc>>,
    #[serde(default)]
    pub company: String,
    #[serde(default)]
    pub admin: String,
}
