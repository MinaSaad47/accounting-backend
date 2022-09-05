use rocket::serde::{Deserialize, Serialize};

use crate::local_storage::rows;

#[derive(Serialize, Deserialize, Default)]
#[serde(crate = "rocket::serde")]
pub struct User {
    #[serde(flatten)]
    pub user: rows::User,
}
