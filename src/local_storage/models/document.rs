use std::path::{Path, PathBuf};

use chrono::{DateTime, Utc};
use rocket::serde::{Deserialize, Serialize};

use crate::file_system::FileSystemFile;

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(crate = "rocket::serde")]
pub struct Document {
    pub path: PathBuf,
    pub name: String,
    pub time: DateTime<Utc>,
}

impl Document {
    pub async fn new(
        company_name: &str,
        file: &(impl FileSystemFile + Sync + ?Sized),
    ) -> Option<Self> {
        let path = Path::new("companies")
            .join(company_name)
            .join("documents")
            .join(file.name_with_ext()?);
        let name = file.name_with_ext()?.to_owned();
        let time = file
            .create_time()
            .await
            .unwrap_or(DateTime::from(Utc::now()));
        Some(Self { path, name, time })
    }
}
