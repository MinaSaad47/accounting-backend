use std::{borrow::Cow, path::Path};

use rocket::{async_trait, fs::TempFile};
use sqlx::types::Uuid;

use crate::local_storage::models::*;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("لم يتم العثور علي هدف")]
    ObjectNotFound,
    #[error("لا يوجد قيمة كافية: \"{0} > {1}\"")]
    NotEnoughUserValue(f64, f64),
    #[error("قيمة غير صحيحة:  >= 0")]
    InvalidValue,
    #[error("حدث خطأ في قاعدة البيانات:\n {0}")]
    Other(Cow<'static, str>),
}

#[async_trait]
pub trait AcountingApi {
    type Company;
    type User;
    type Expense;
    type Income;
    type Document;
    type Funder;
    type Error;

    async fn create_company(&self, c: &CreateCompany) -> Result<Self::Company, Error>;

    async fn update_company(&self, id: Uuid, c: &UpdateCompany) -> Result<Self::Company, Error>;

    async fn search_company(&self, s: &str) -> Result<Vec<Self::Company>, Error>;

    async fn pay_company(&self, c: &Self::Company, v: f64) -> Result<Self::Company, Error>;

    async fn delete_company(&self, id: Uuid) -> Result<(), Error>;

    async fn create_funder(
        &self,
        company_id: Uuid,
        f: &CreateFunder,
    ) -> Result<Self::Funder, Error>;
    async fn get_funders(&self, company_id: Uuid) -> Result<Vec<Self::Funder>, Error>;
    async fn delete_funder(&self, id: Uuid) -> Result<(), Error>;

    async fn register_user(&self, u: &RegisterUser) -> Result<Self::User, Error>;

    async fn update_user(&self, id: Uuid, u: &UpdateUser) -> Result<Self::User, Error>;

    async fn get_users(&self) -> Result<Vec<Self::User>, Error>;

    async fn pay_user(&self, id: Uuid, v: f64) -> Result<Self::User, Error>;

    async fn get_user(&self, id: Uuid) -> Result<Self::User, Error>;

    async fn login_user(&self, u: &LoginUser) -> Result<Self::User, Error>;

    async fn delete_user(&self, id: Uuid) -> Result<(), Error>;

    async fn get_expenses(
        &self,
        user_id: Option<Uuid>,
        company_id: Option<Uuid>,
    ) -> Result<Vec<Self::Expense>, Error>;

    async fn create_expense(
        &self,
        user_id: Uuid,
        company_id: Uuid,
        expense: &CreateExpense,
    ) -> Result<Self::Expense, Error>;

    async fn delete_expense(&self, id: Uuid) -> Result<(), Error>;

    async fn get_incomes(
        &self,
        admin_id: Option<Uuid>,
        company_id: Option<Uuid>,
    ) -> Result<Vec<Self::Income>, Error>;

    async fn create_income(
        &self,
        admin_id: Uuid,
        company_id: Uuid,
        income: &CreateIncome,
    ) -> Result<Self::Income, Error>;

    async fn delete_income(&self, id: Uuid) -> Result<(), Error>;

    async fn create_document(
        &self,
        company_id: Uuid,
        file: &mut TempFile<'_>,
    ) -> Result<Self::Document, Error>;

    async fn get_documents(&self, company_id: Uuid) -> Result<Vec<Self::Document>, Error>;
    async fn delete_document(&self, path: impl AsRef<Path> + Send) -> Result<(), Error>;
}
