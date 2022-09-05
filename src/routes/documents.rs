use std::path::{Path, PathBuf};

use rocket::{
    delete,
    fairing::AdHoc,
    fs::{relative, NamedFile},
    get, routes, State,
};

use crate::{
    accounting_api::AcountingApi,
    auth::AGuard,
    local_storage::LocalStorageAccountingApi,
    types::response::{ResponseEnum, ResponseResult},
};

#[get("/<path..>")]
pub async fn download_document(path: PathBuf) -> Option<NamedFile> {
    let path = Path::new(relative!("db/file_system")).join(path);
    NamedFile::open(path).await.ok()
}

#[delete("/<id>")]
pub async fn delete_document(
    id: i64,
    storage: &State<LocalStorageAccountingApi>,
    _ag: AGuard,
) -> ResponseResult<()> {
    storage.delete_document(id).await?;
    Ok(ResponseEnum::ok((), "تم مسح المستند".into()))
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("documents stage", |rocket| async {
        rocket.mount(
            "/api/documents",
            routes![download_document, delete_document],
        )
    })
}
