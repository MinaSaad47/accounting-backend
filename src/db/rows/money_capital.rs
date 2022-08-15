use rocket::serde::{Serialize, Deserialize};
use sqlx::FromRow;

use crate::{types::{uuid::Uuid, date::Date}, querable_struct};

querable_struct! {
    #[derive(Serialize, Deserialize, FromRow)]
    #[serde(crate = "rocket::serde")]
    pub struct MoneyCapital {
        #[serde(default)]
        pub id: Option<Uuid>,
        pub value: f64,
        pub time: Date,
        #[serde(default)]
        pub company_id: Uuid,
    }
}
