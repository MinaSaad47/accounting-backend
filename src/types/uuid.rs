use std::{
    borrow::Cow,
    error::Error,
    ops::{Deref, DerefMut},
    str::FromStr,
};

use rocket::serde::{Deserialize, Serialize};
use sqlx::{
    database::HasValueRef,
    encode::IsNull,
    sqlite::{SqliteArgumentValue, SqliteTypeInfo},
    Database, Decode, Encode, FromRow, Sqlite, Type,
};

#[derive(Serialize, Deserialize, Default, FromRow)]
#[sqlx(transparent)]
#[serde(crate = "rocket::serde")]
pub struct Uuid(pub uuid::Uuid);

impl Uuid {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}

impl Deref for Uuid {
    type Target = uuid::Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Uuid {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// just to fix:
// "error occurred while decoding column 0: invalid length: expected 16 bytes, found 36"
impl Type<Sqlite> for Uuid {
    fn type_info() -> SqliteTypeInfo {
        <&str as Type<Sqlite>>::type_info()
    }
}

impl<'q> Encode<'q, Sqlite> for Uuid {
    fn encode(self, args: &mut Vec<SqliteArgumentValue<'q>>) -> IsNull {
        args.push(SqliteArgumentValue::Text(Cow::Owned(self.0.to_string())));

        IsNull::No
    }

    fn encode_by_ref(&self, args: &mut Vec<SqliteArgumentValue<'q>>) -> IsNull {
        args.push(SqliteArgumentValue::Text(Cow::Owned(self.0.clone().to_string())));

        IsNull::No
    }
}

impl<'r, DB: Database> Decode<'r, DB> for Uuid
where
    &'r str: Decode<'r, DB>,
{
    fn decode(
        value: <DB as HasValueRef<'r>>::ValueRef,
    ) -> Result<Uuid, Box<dyn Error + 'static + Send + Sync>> {
        let value = <&str as Decode<DB>>::decode(value)?;

        Ok(Uuid(uuid::Uuid::from_str(value)?))
    }
}
