use std::{
    io,
    path::{Path, PathBuf},
};

use rocket::{fs::TempFile, tokio::fs};

#[derive(Debug)]
pub struct FileSystem {
    pub root: PathBuf,
}

impl FileSystem {
    pub async fn new(root: impl AsRef<Path>) -> Self {
        if !root.as_ref().exists() {
            fs::create_dir_all(&root)
                .await
                .expect("valid directory and permissions")
        }
        Self {
            root: root.as_ref().to_owned(),
        }
    }

    pub async fn save(
        &mut self,
        path: impl AsRef<Path>,
        file: &mut TempFile<'_>,
    ) -> io::Result<()> {
        let path = self.root.join(path.as_ref());
        match path.parent() {
            Some(parent) => {
                rocket::info!("checking directory `{:?}`", parent);
                if !parent.exists() {
                    rocket::warn!("directory not found `{:?}`", parent);
                    rocket::info!("creating directory `{:?}`", parent);
                    fs::create_dir_all(parent).await?;
                    rocket::info!("created directory `{:?}`", parent);
                } else {
                    rocket::info!("found directory `{:?}`", parent);
                }
            }
            None => {
                rocket::error!("could not create parent directory `{:?}`", path);
            }
        }
        file.move_copy_to(path).await?;
        Ok(())
    }

    pub async fn delete(&mut self, path: impl AsRef<Path>) -> io::Result<()> {
        let path = self.root.join(path.as_ref());
        fs::remove_file(path).await?;
        Ok(())
    }
}
