use rocket::{fairing::AdHoc, get, post, put, routes, serde::json::Json, State};

use crate::{
    accounting_api::AcountingApi,
    auth::{AGuard, UGuard},
    database::{models,rows, DatabaseAccountingApi},
    types::response::{ResponseEnum, ResponseResult},
};

#[post("/", format = "application/json", data = "<company>")]
pub async fn create_company(
    company: Json<models::Company>,
    storage: &State<DatabaseAccountingApi>,
    _ag: AGuard,
) -> ResponseResult<models::Company> {
    rocket::trace!("{company:#?}");
    let company = storage.create_company(&company.into_inner()).await?;
    Ok(ResponseEnum::created(company, "تم انشاء شركة جديدة".into()))
}

#[get("/?<search>")]
pub async fn search_company_admin(
    search: &str,
    storage: &State<DatabaseAccountingApi>,
    _ag: AGuard,
) -> ResponseResult<Vec<models::Company>> {
    rocket::trace!("{search:#?}");
    let companies = storage.search_company(search).await?;
    Ok(ResponseEnum::ok(companies, "تم العثور علي شركات".into()))
}

#[get("/?<search>", rank = 2)]
pub async fn search_company_user(
    search: &str,
    storage: &State<DatabaseAccountingApi>,
    _ug: UGuard,
) -> ResponseResult<Vec<models::Company>> {
    rocket::trace!("{search:#?}");
    let companies = storage.search_company(search).await?;
    Ok(ResponseEnum::ok(companies, "تم العثور علي شركات".into()))
}

#[put("/", format = "application/json", data = "<company>")]
pub async fn update_company(
    company: Json<models::Company>,
    storage: &State<DatabaseAccountingApi>,
    _ag: AGuard,
) -> ResponseResult<models::Company> {
    rocket::trace!("{company:#?}");
    let compannies = storage.update_company(&mut company.into_inner()).await?;
    Ok(ResponseEnum::ok(compannies, "تم خفظ الشركة بنجاح".into()))
}

#[post(
    "/<company_id>/money_capitals",
    format = "application/json",
    data = "<money_capital>"
)]
pub async fn create_money_capital(
    company_id: i64,
    money_capital: Json<rows::MoneyCapital>,
    storage: &State<DatabaseAccountingApi>,
    ug: UGuard,
) -> ResponseResult<models::MoneyCapital> {
    let money_capital = storage
        .create_money_capital(ug.0, company_id, money_capital.value)
        .await?;

    Ok(ResponseEnum::created(
        money_capital,
        "تم اضافة رأس مال".into(),
    ))
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("companies stage", |rocket| async {
        rocket.mount(
            "/api/company",
            routes![
                create_company,
                update_company,
                search_company_admin,
                search_company_user,
                create_money_capital,
            ],
        )
    })
}
