use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{:#?} خطأ في قاعدة البيانات", source)]
    DataBase {
        #[from]
        source: sqlx::Error,
    },
    #[error("{:#?} حدث خطأ في السيرفر", source)]
    Backend {
        #[from]
        source: rocket::Error,
    },
}

