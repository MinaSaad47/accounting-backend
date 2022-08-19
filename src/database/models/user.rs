use rocket::serde::{Deserialize, Serialize};

use crate::database::rows;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct User {
    #[serde(flatten)]
    pub user: rows::User,
}
