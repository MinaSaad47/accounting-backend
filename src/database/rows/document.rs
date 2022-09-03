use std::path::PathBuf;

use chrono::{DateTime, Utc};
use rocket::serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Default, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Document {
    pub id: i64,
    pub name: String,
    pub time: DateTime<Utc>,
    pub company_id: i64,
}

impl From<&Document> for PathBuf {
    fn from(document: &Document) -> Self {
        Self::from(document.company_id.to_string())
            .join(format!("{}_{}", document.id, document.name))
    }
}
