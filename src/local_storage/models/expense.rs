use rocket::serde::{Serialize, Deserialize};
use sqlx::FromRow;

use crate::local_storage::rows;



#[derive(Serialize, Deserialize, FromRow, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Expense {
    pub company: String,
    pub user: String,
    #[serde(flatten)]
    pub expense: rows::Expense,
}
