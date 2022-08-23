use rocket::serde::{Deserialize, Serialize};

use crate::database::rows;

#[derive(Serialize, Deserialize, Default)]
#[serde(crate = "rocket::serde")]
pub struct User {
    #[serde(flatten)]
    pub user: rows::User,
}
