use rocket::serde::{Serialize, Deserialize};

use crate::types::uuid::Uuid;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Funder {
    pub name: String,
    #[serde(default)]
    pub company_id: Uuid,
}
