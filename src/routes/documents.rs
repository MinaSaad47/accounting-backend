use std::{
    io,
    path::{Path, PathBuf},
};

use rocket::{
    delete,
    fairing::AdHoc,
    fs::NamedFile,
    get, routes,
    serde::{json::Json, Deserialize, Serialize},
    State,
};

use crate::{
    accounting_api::AcountingApi,
    auth::{AGuard, UGuard},
    local_storage::LocalStorageAccountingApi,
    types::response::{ResponseEnum, ResponseResult},
};

#[get("/<path..>")]
pub async fn download_document_admin(
    path: PathBuf,
    storage: &State<LocalStorageAccountingApi>,
    _ag: AGuard,
) -> io::Result<NamedFile> {
    let path = Path::new(&storage.fs.read().await.root).join(path);
    rocket::info!("[admin|documents] requesting: {path:?}");
    NamedFile::open(path).await
}

#[get("/<path..>", rank = 2)]
pub async fn download_document_user(
    path: PathBuf,
    storage: &State<LocalStorageAccountingApi>,
    _ug: UGuard,
) -> io::Result<NamedFile> {
    let path = Path::new(&storage.fs.read().await.root).join(path);
    rocket::info!("[user|documents] requesting: {path:?}");
    NamedFile::open(path).await
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct DeleteData {
    pub path: PathBuf,
}

#[delete("/", format = "application/json", data = "<data>")]
pub async fn delete_document(
    data: Json<DeleteData>,
    storage: &State<LocalStorageAccountingApi>,
    _ag: AGuard,
) -> ResponseResult<()> {
    storage.delete_document(data.into_inner().path).await?;
    Ok(ResponseEnum::ok((), "تم مسح المستند".into()))
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("documents stage", |rocket| async {
        rocket.mount(
            "/api/documents",
            routes![
                download_document_admin,
                download_document_user,
                delete_document,
            ],
        )
    })
}
