use rocket::serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
#[serde(crate = "rocket::serde", rename = "camelCase")]
pub struct Funder {
    #[serde(default)]
    pub id: Option<i64>,
    pub name: String,
    #[serde(default)]
    pub company_id: i64,
}
