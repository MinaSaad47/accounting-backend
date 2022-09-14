use std::{
    borrow::Cow,
    ffi::OsStr,
    io,
    path::{Path, PathBuf},
};

use chrono::{DateTime, Utc};
use rocket::{async_trait, fs::TempFile, tokio::fs};

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
        file: impl FileSystemFile,
    ) -> io::Result<()> {
        let path = self.root.join(path.as_ref());
        rocket::trace!("[save] saving {:?}", path);
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
        file.save_to(path).await?;
        Ok(())
    }

    pub async fn delete(&mut self, path: impl AsRef<Path> + Send) -> io::Result<()> {
        let path = self.root.join(path.as_ref());
        rocket::trace!("[delete] deleting {:?}", path);
        fs::remove_file(path).await?;
        Ok(())
    }

    pub async fn get(&self, path: impl AsRef<Path>) -> Vec<Cow<'static, Path>> {
        let mut files = Vec::new();
        let path = self.root.join(path);
        rocket::info!("FileSystem::get({path:?})");
        for entry in path.read_dir().into_iter().flatten() {
            if let Ok(entry) = entry {
                let path = entry.path();
                rocket::info!("\t Found {path:?}");
                files.push(path.into());
            }
        }
        files
    }
}

#[async_trait]
pub trait FileSystemFile {
    async fn save_to(self, path: impl AsRef<Path> + Send) -> io::Result<()>;
    fn name_with_ext(&self) -> Option<&str>;
    fn name_without_ext(&self) -> Option<&str>;
    async fn create_time(&self) -> Option<DateTime<Utc>> {
        None
    }
}

#[async_trait]
impl FileSystemFile for &mut TempFile<'_> {
    async fn save_to(self, path: impl AsRef<Path> + Send) -> io::Result<()> {
        self.move_copy_to(path).await
    }

    fn name_with_ext(&self) -> Option<&str> {
        self.raw_name()
            .map(|f| f.dangerous_unsafe_unsanitized_raw().as_str())
    }

    fn name_without_ext(&self) -> Option<&str> {
        self.name()
    }
}

#[async_trait]
impl FileSystemFile for &Path {
    async fn save_to(self, path: impl AsRef<Path> + Send) -> io::Result<()> {
        fs::rename(self, path).await
    }

    fn name_with_ext(&self) -> Option<&str> {
        self.file_name().and_then(|f| f.to_str())
    }
    fn name_without_ext(&self) -> Option<&str> {
        self.file_stem()
            .and_then(|f| <OsStr as AsRef<Path>>::as_ref(f).file_name())
            .and_then(|f| f.to_str())
    }
    async fn create_time(&self) -> Option<DateTime<Utc>> {
        fs::metadata(self)
            .await
            .and_then(|md| md.created())
            .ok()
            .map(|ct| ct.into())
    }
}
