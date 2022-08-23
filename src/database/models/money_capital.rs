use rocket::serde::{Serialize, Deserialize};
use sqlx::FromRow;

use crate::database::rows;



#[derive(Serialize, Deserialize, FromRow, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct MoneyCapital {
    pub company: String,
    pub user: String,
    #[serde(flatten)]
    pub money_capital: rows::MoneyCapital,
}

