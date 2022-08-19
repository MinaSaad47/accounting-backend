use rocket::serde::{Deserialize, Serialize};

use crate::database::rows;

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(crate = "rocket::serde")]
pub struct Company {
    #[serde(flatten)]
    pub company: rows::Company,
    pub funders: Vec<rows::Funder>,
    pub money_capitals: Option<Vec<rows::MoneyCapital>>,
}
