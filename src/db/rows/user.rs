use rocket::serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::{types::uuid::Uuid, querable_struct};

querable_struct! {
    #[derive(Serialize, Deserialize, FromRow)]
    #[serde(crate = "rocket::serde")]
    pub struct User {
        #[serde(default)]
        pub id: Option<Uuid>,
        pub name: String,
        pub password: String,
    }
}
