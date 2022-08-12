use std::ops::{Deref, DerefMut};

use rocket::serde::{Deserialize, Serialize};

use sqlx::{types::chrono::NaiveDateTime, Type};

type SqlxDate = NaiveDateTime;

#[derive(Serialize, Deserialize, Clone, Copy, Type, Default)]
#[serde(crate = "rocket::serde")]
#[sqlx(transparent)]
pub struct Date(pub SqlxDate);

impl Deref for Date {
    type Target = SqlxDate;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Date {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
