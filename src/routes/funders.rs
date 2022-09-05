use rocket::{delete, fairing::AdHoc, routes, State};

use crate::{
    accounting_api::AcountingApi,
    auth::AGuard,
    local_storage::LocalStorageAccountingApi,
    types::response::{ResponseEnum, ResponseResult},
};

#[delete("/<id>")]
pub async fn delete_funder(
    id: i64,
    storage: &State<LocalStorageAccountingApi>,
    _ag: AGuard,
) -> ResponseResult<()> {
    storage.delete_funder(id).await?;
    Ok(ResponseEnum::ok((), "تم مسح الممول بنجاح".into()))
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("funders stage", |rocket| async {
        rocket.mount("/api/funders", routes![delete_funder,])
    })
}
