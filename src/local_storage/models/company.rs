use rocket::serde::{Deserialize, Serialize};

use crate::local_storage::rows;

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(crate = "rocket::serde")]
pub struct Company {
    #[serde(flatten)]
    pub company: rows::Company,
}

impl From<rows::Company> for Company {
    fn from(company: rows::Company) -> Self {
        Self { company }
    }
}

impl AsRef<rows::Company> for Company {
    fn as_ref(&self) -> &rows::Company {
        &self.company
    }
}
