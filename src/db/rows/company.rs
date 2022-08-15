use crate::{types::{uuid::Uuid, date::Date}, querable_struct};
use rocket::serde::{Deserialize, Serialize};
use sqlx::FromRow;

querable_struct! {
    #[derive(Serialize, Deserialize, FromRow, Default)]
    #[serde(crate = "rocket::serde")]
    pub struct Company {
        pub id: Option<Uuid>,
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
}
