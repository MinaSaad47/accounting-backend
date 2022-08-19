use rocket::async_trait;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("لم يتم العثور علي هدف")]
    ObjectNotFound,
    #[error("لم يتم العثور علي مستخدم")]
    Other
}

#[async_trait]
pub trait AcountingApi {
    type Company;
    type User;
    async fn create_company(&self, c: &Self::Company) -> Result<Self::Company, Error>;
    async fn update_company(&self, c: &mut Self::Company) -> Result<Self::Company, Error>;
    async fn search_company(&self, s: &str) -> Result<Vec<Self::Company>, Error>;
    async fn pay_company(&self, c: &Self::Company, v: f64) -> Result<Self::Company, Error>;

    async fn create_user(&self, c: &Self::User) -> Result<Self::User, Error>;
    async fn update_user(&self, c: &Self::User) -> Result<Self::User, Error>;
    async fn search_user(&self, s: &str) -> Result<Vec<Self::User>, Error>;
    async fn pay_user(&self, c: &Self::User, v: f64) -> Result<Self::User, Error>;
}
