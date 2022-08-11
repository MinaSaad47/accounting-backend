use crate::types::{uuid::Uuid, date::Date};
use rocket::serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
#[serde(crate = "rocket::serde")]
pub struct Company {
    #[serde(default)]
    pub id: Uuid,
    pub commercial_feature: String,
    pub is_working: bool,
    pub legal_entity: String,
    pub file_number: Option<String>,
    pub register_number: String,
    pub start_date: Date,
    pub stop_date: Option<Date>,
    pub general_tax_mission: String,
    pub value_tax_mission: Option<String>,
    pub activity_nature: String,
    pub activity_location: String,
    pub accounts: String,
    pub joining_date: Option<Date>,
    pub natural_id: Option<String>,
    pub money_capital: Option<f64>,
    pub record_side: Option<String>,
    pub record_number: String,
    pub user_name: String,
    pub passport: Option<String>,
    pub verification_code: Option<String>,
    pub email: String,
}

impl Company {
    pub fn new(
        id: Uuid,
        commercial_feature: String,
        is_working: bool,
        legal_entity: String,
        register_number: String,
        start_date: Date,
        general_tax_mission: String,
        activity_nature: String,
        activity_location: String,
        accounts: String,
        record_number: String,
        user_name: String,
        email: String,
    ) -> Self {
        Self {
            id,
            commercial_feature,
            is_working,
            legal_entity,
            file_number: None,
            register_number,
            start_date,
            stop_date: None,
            general_tax_mission,
            value_tax_mission: None,
            activity_nature,
            activity_location,
            accounts,
            joining_date: None,
            natural_id: None,
            money_capital: None,
            record_side: None,
            record_number,
            user_name,
            passport: None,
            verification_code: None,
            email,
        }
    }
}
