use rocket::serde::{Serialize, Deserialize};
use sqlx::FromRow;

use crate::local_storage::rows;



#[derive(Serialize, Deserialize, FromRow, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Income {
    pub company: String,
    pub admin: String,
    #[serde(flatten)]
    pub income: rows::Income,
}

