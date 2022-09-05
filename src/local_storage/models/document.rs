use std::path::PathBuf;

use rocket::serde::{Deserialize, Serialize};

use crate::local_storage::rows;

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(crate = "rocket::serde")]
pub struct Document {
    pub path: PathBuf,
    #[serde(flatten)]
    pub document: rows::Document,
}

impl From<rows::Document> for Document {
    fn from(document: rows::Document) -> Self {
        Self {
            path: document.to_path_buf(),
            document,
        }
    }
}

impl AsRef<rows::Document> for Document {
    fn as_ref(&self) -> &rows::Document {
        &self.document
    }
}
