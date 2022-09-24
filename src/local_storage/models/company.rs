use chrono::Utc;
use rocket::serde::{Deserialize, Serialize};
use sqlx::types::chrono::DateTime;

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub struct Company {
    pub id: Option<i64>,
    pub commercial_feature: String,
    pub is_working: bool,
    pub legal_entity: Option<String>,
    pub file_number: Option<String>,
    pub register_number: Option<String>,
    pub start_date: Option<DateTime<Utc>>,
    pub stop_date: Option<DateTime<Utc>>,
    pub general_tax_mission: Option<String>,
    pub value_tax_mission: Option<String>,
    pub activity_nature: Option<String>,
    pub activity_location: Option<String>,
    pub record_number: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub email: Option<String>,
}
