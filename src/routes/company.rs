use rocket::{fairing::AdHoc, get, post, put, routes, serde::json::Json, State};

use crate::{
    accounting_api::AcountingApi,
    database::{models, DatabaseAccountingApi},
    types::response::{ResponseEnum, ResponseResult},
};

#[post("/", format = "application/json", data = "<company>")]
pub async fn create_company(
    company: Json<models::Company>,
    storage: &State<DatabaseAccountingApi>,
) -> ResponseResult<models::Company> {
    rocket::trace!("{company:#?}");
    let company = storage.create_company(&company.into_inner()).await?;
    Ok(ResponseEnum::created(company, "تم انشاء شركة جديدة".into()))
}

#[get("/?<search>")]
pub async fn search_company(
    search: &str,
    storage: &State<DatabaseAccountingApi>,
) -> ResponseResult<Vec<models::Company>> {
    rocket::trace!("{company:#?}");
    let companies = storage.search_company(search).await?;
    Ok(ResponseEnum::ok(companies, "تم العثور علي شركات".into()))
}

#[put("/", format = "application/json", data = "<company>")]
pub async fn update_company(
    company: Json<models::Company>,
    storage: &State<DatabaseAccountingApi>,
) -> ResponseResult<models::Company> {
    rocket::trace!("{company:#?}");
    let compannies = storage.update_company(&mut company.into_inner()).await?;
    Ok(ResponseEnum::ok(compannies, "تم خفظ الشركة بنجاح".into()))
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("companies stage", |rocket| async {
        rocket.mount(
            "/api/company",
            routes![create_company, update_company, search_company],
        )
    })
}
