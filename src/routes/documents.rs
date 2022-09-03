use std::path::{Path, PathBuf};

use rocket::{
    fairing::AdHoc,
    fs::{relative, NamedFile},
    get, routes,
};


#[get("/<path..>")]
pub async fn download_document(path: PathBuf) -> Option<NamedFile> {
    let path = Path::new(relative!("db/documents")).join(path);
    NamedFile::open(path).await.ok()
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("documents stage", |rocket| async {
        rocket.mount("/api/documents", routes![download_document])
    })
}
