use std::borrow::Cow;

use rocket::{async_trait, data::DataStream};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("لم يتم العثور علي هدف")]
    ObjectNotFound,
    #[error("لا يوجد قيمة كافية: \"{0} > {1}\"")]
    NotEnoughUserValue(f64, f64),
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
    type Error;

    async fn create_company(&self, c: &Self::Company) -> Result<Self::Company, Error>;

    async fn update_company(&self, c: &mut Self::Company) -> Result<Self::Company, Error>;

    async fn search_company(&self, s: &str) -> Result<Vec<Self::Company>, Error>;

    async fn pay_company(&self, c: &Self::Company, v: f64) -> Result<Self::Company, Error>;

    async fn delete_company(&self, id: i64) -> Result<(), Error>;

    async fn create_user(&self, u: &Self::User) -> Result<Self::User, Error>;

    async fn update_user(&self, u: &Self::User) -> Result<Self::User, Error>;

    async fn get_users(&self) -> Result<Vec<Self::User>, Error>;

    async fn pay_user(&self, id: i64, v: f64) -> Result<Self::User, Error>;

    async fn get_user(&self, id: i64) -> Result<Self::User, Error>;

    async fn login_user(&self, u: &Self::User) -> Result<Self::User, Error>;

    async fn delete_user(&self, id: i64) -> Result<(), Error>;

    async fn get_expenses(
        &self,
        user_id: Option<i64>,
        company_id: Option<i64>,
    ) -> Result<Vec<Self::Expense>, Error>;

    async fn create_expense(
        &self,
        user_id: i64,
        company_id: i64,
        value: f64,
        description: &str,
    ) -> Result<Self::Expense, Error>;

    async fn delete_expense(&self, id: i64) -> Result<(), Error>;

    async fn get_incomes(
        &self,
        admin_id: Option<i64>,
        company_id: Option<i64>,
    ) -> Result<Vec<Self::Income>, Error>;

    async fn create_income(
        &self,
        admin_id: i64,
        company_id: i64,
        value: f64,
        description: &str,
    ) -> Result<Self::Income, Error>;

    async fn delete_income(&self, id: i64) -> Result<(), Error>;

    async fn create_document(
        &self,
        company_id: i64,
        name: &str,
        data: DataStream<'_>,
    ) -> Result<Self::Document, Error>;

    async fn get_documents(&self, company_id: i64) -> Result<Vec<Self::Document>, Error>;
}
