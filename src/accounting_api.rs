use rocket::async_trait;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("لم يتم العثور علي هدف")]
    ObjectNotFound,
    #[error("لا يوجد قيمة كافية: {0} < {1}`")]
    NotEnoughUserValue(f64, f64),
    #[error("لم يتم العثور علي مستخدم")]
    Other,
}

#[async_trait]
pub trait AcountingApi {
    type Company;
    type User;
    type MoneyCapital;
    type Error;
    async fn create_company(&self, c: &Self::Company) -> Result<Self::Company, Error>;
    async fn update_company(&self, c: &mut Self::Company) -> Result<Self::Company, Error>;
    async fn search_company(&self, s: &str) -> Result<Vec<Self::Company>, Error>;
    async fn pay_company(&self, c: &Self::Company, v: f64) -> Result<Self::Company, Error>;

    async fn create_user(&self, u: &Self::User) -> Result<Self::User, Error>;
    async fn update_user(&self, u: &Self::User) -> Result<Self::User, Error>;
    async fn get_users(&self) -> Result<Vec<Self::User>, Error>;
    async fn pay_user(&self, u: &Self::User, v: f64) -> Result<Self::User, Error>;
    async fn get_user(&self, id: i64) -> Result<Self::User, Error>;
    async fn login_user(&self, u: &Self::User) -> Result<Self::User, Error>;

    async fn get_money_capitals(
        &self,
        user_id: Option<i64>,
        company_id: Option<i64>,
    ) -> Result<Vec<Self::MoneyCapital>, Error>;
    async fn create_money_capital(
        &self,
        user_id: i64,
        company_id: i64,
        value: f64,
        description: &str,
    ) -> Result<Self::MoneyCapital, Error>;
    async fn delete_money_capital(
        &self,
        id: i64,
    ) -> Result<(), Error>;
}
