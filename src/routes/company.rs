use rocket::{
    delete, fairing::AdHoc, form::Form, fs::TempFile, get, post, put, routes, serde::json::Json,
    FromForm, State,
};

use crate::{
    accounting_api::AcountingApi,
    auth::{AGuard, UGuard},
    local_storage::{models::*, LocalStorageAccountingApi},
    types::response::{ResponseEnum, ResponseResult},
};

#[post("/", format = "application/json", data = "<company>")]
pub async fn create_company(
    company: Json<CreateCompany>,
    storage: &State<LocalStorageAccountingApi>,
    _ag: AGuard,
) -> ResponseResult<Company> {
    rocket::trace!("{company:#?}");
    let company = storage.create_company(&company).await?;
    Ok(ResponseEnum::created(company, "تم انشاء شركة جديدة".into()))
}

#[get("/?<search>")]
pub async fn search_company_admin(
    search: &str,
    storage: &State<LocalStorageAccountingApi>,
    _ag: AGuard,
) -> ResponseResult<Vec<Company>> {
    rocket::trace!("{search:#?}");
    let companies = storage.search_company(search).await?;
    Ok(ResponseEnum::ok(companies, "تم العثور علي شركات".into()))
}

#[get("/?<search>", rank = 2)]
pub async fn search_company_user(
    search: &str,
    storage: &State<LocalStorageAccountingApi>,
    _ug: UGuard,
) -> ResponseResult<Vec<Company>> {
    rocket::trace!("{search:#?}");
    let companies = storage.search_company(search).await?;
    Ok(ResponseEnum::ok(companies, "تم العثور علي شركات".into()))
}

#[put("/<id>", format = "application/json", data = "<company>")]
pub async fn update_company(
    id: i64,
    company: Json<UpdateCompany>,
    storage: &State<LocalStorageAccountingApi>,
    _ag: AGuard,
) -> ResponseResult<Company> {
    rocket::trace!("{company:#?}");
    let compannies = storage.update_company(id, &company).await?;
    Ok(ResponseEnum::ok(compannies, "تم خفظ الشركة بنجاح".into()))
}

#[post(
    "/<company_id>/expenses",
    format = "application/json",
    data = "<expense>"
)]
pub async fn create_expense(
    company_id: i64,
    expense: Json<CreateExpense>,
    storage: &State<LocalStorageAccountingApi>,
    ug: UGuard,
) -> ResponseResult<Expense> {
    let expense = storage.create_expense(ug.0, company_id, &expense).await?;

    Ok(ResponseEnum::created(expense, "تم اضافة مصروفات".into()))
}

#[post(
    "/<company_id>/incomes",
    format = "application/json",
    data = "<income>"
)]
pub async fn create_income(
    company_id: i64,
    income: Json<CreateIncome>,
    storage: &State<LocalStorageAccountingApi>,
    ag: AGuard,
) -> ResponseResult<Income> {
    let income = storage.create_income(ag.0, company_id, &income).await?;

    Ok(ResponseEnum::created(income, "تم اضافة واردات".into()))
}

#[delete("/<id>")]
pub async fn delete_company(
    id: i64,
    storage: &State<LocalStorageAccountingApi>,
    _ag: AGuard,
) -> ResponseResult<()> {
    storage.delete_company(id).await?;
    Ok(ResponseEnum::ok((), "تم حذف الشركة".into()))
}

#[derive(FromForm, Debug)]
struct Upload<'r> {
    file: TempFile<'r>,
}

#[post("/<company_id>/documents", data = "<upload>")]
async fn upload_document(
    company_id: i64,
    mut upload: Form<Upload<'_>>,
    storage: &State<LocalStorageAccountingApi>,
) -> ResponseResult<Document> {
    let document = storage
        .create_document(company_id, &mut upload.file)
        .await?;

    Ok(ResponseEnum::created(
        document,
        "تم انشاء مستند بنجاح".into(),
    ))
}

#[get("/<company_id>/documents")]
async fn get_documents_admin(
    company_id: i64,
    storage: &State<LocalStorageAccountingApi>,
    _ag: AGuard,
) -> ResponseResult<Vec<Document>> {
    let documents = storage.get_documents(company_id).await?;
    Ok(ResponseEnum::ok(documents, "تم ايجاد مستندات بنجاح".into()))
}

#[get("/<company_id>/documents", rank = 2)]
async fn get_documents_user(
    company_id: i64,
    storage: &State<LocalStorageAccountingApi>,
    _ug: UGuard,
) -> ResponseResult<Vec<Document>> {
    let documents = storage.get_documents(company_id).await?;
    Ok(ResponseEnum::ok(documents, "تم ايجاد مستندات بنجاح".into()))
}

#[post(
    "/<company_id>/funders",
    format = "application/json",
    data = "<funder>"
)]
async fn create_funder(
    company_id: i64,
    funder: Json<CreateFunder>,
    storage: &State<LocalStorageAccountingApi>,
    _ag: AGuard,
) -> ResponseResult<Funder> {
    let funder = storage.create_funder(company_id, &funder).await?;
    Ok(ResponseEnum::created(funder, "تم اضافة ممول ببنجاح".into()))
}

#[get("/<company_id>/funders")]
async fn get_funders_admin(
    company_id: i64,
    storage: &State<LocalStorageAccountingApi>,
    _ag: AGuard,
) -> ResponseResult<Vec<Funder>> {
    let funder = storage.get_funders(company_id).await?;
    Ok(ResponseEnum::created(funder, "تم اضافة ممول ببنجاح".into()))
}

#[get("/<company_id>/funders", rank = 2)]
async fn get_funders_user(
    company_id: i64,
    storage: &State<LocalStorageAccountingApi>,
    _ug: UGuard,
) -> ResponseResult<Vec<Funder>> {
    let funder = storage.get_funders(company_id).await?;
    Ok(ResponseEnum::created(funder, "تم اضافة ممول ببنجاح".into()))
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
                create_expense,
                create_income,
                delete_company,
                upload_document,
                get_documents_admin,
                get_documents_user,
                create_funder,
                get_funders_admin,
                get_funders_user,
            ],
        )
    })
}
