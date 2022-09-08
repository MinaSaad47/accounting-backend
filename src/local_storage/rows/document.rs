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
    #[serde(skip)]
    pub company_name: String,
}

impl Document {
    pub fn to_path_buf(&self) -> PathBuf {
        PathBuf::from("companies")
            .join(format!(
                "{}_{}",
                self.company_id.to_string(),
                self.company_name
            ))
            .join("documents")
            .join(format!("{}_{}", self.id, self.name))
    }
}
