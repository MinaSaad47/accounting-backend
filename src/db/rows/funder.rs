use rocket::serde::{Serialize, Deserialize};
use sqlx::FromRow;

use crate::{types::uuid::Uuid, querable_struct};

querable_struct! {
    #[derive(Serialize, Deserialize, FromRow)]
    #[serde(crate = "rocket::serde")]
    pub struct Funder {
        #[serde(default)]
        pub id: Option<Uuid>,
        pub name: String,
        #[serde(default)]
        pub company_id: Uuid,
    }
}
