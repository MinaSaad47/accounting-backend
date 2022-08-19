use chrono::Utc;
use rocket::serde::{Deserialize, Serialize};
use sqlx::types::chrono::DateTime;
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Default, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Company {
    pub id: Option<i64>,
    pub commercial_feature: String,
    pub is_working: bool,
    pub legal_entity: String,
    pub file_number: Option<String>,
    pub register_number: String,
    pub start_date: DateTime<Utc>,
    pub stop_date: Option<DateTime<Utc>>,
    pub general_tax_mission: String,
    pub value_tax_mission: Option<String>,
    pub activity_nature: String,
    pub activity_location: String,
    pub accounts: String,
    pub joining_date: Option<DateTime<Utc>>,
    pub natural_id: Option<String>,
    pub money_capital: Option<f64>,
    pub record_side: Option<String>,
    pub record_number: String,
    pub user_name: String,
    pub passport: Option<String>,
    pub verification_code: Option<String>,
    pub email: String,
}
