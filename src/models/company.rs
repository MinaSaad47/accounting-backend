use crate::db::rows;
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Company {
    #[serde(flatten)]
    pub company: rows::Company,
    pub funders: Vec<rows::Funder>,
    pub money_capitals: Option<Vec<rows::MoneyCapital>>,
}
