use rocket::serde::{Deserialize, Serialize};

use crate::db::rows;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct User {
    #[serde(flatten)]
    pub user: rows::User,
}
