use rocket::serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::local_storage::rows;

#[derive(Serialize, Deserialize, FromRow, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Funder {
    #[serde(flatten)]
    funder: rows::Funder,
}

impl From<rows::Funder> for Funder {
    fn from(funder: rows::Funder) -> Self {
        Self { funder }
    }
}

impl AsRef<rows::Funder> for Funder {
    fn as_ref(&self) -> &rows::Funder {
        &self.funder
    }
}
