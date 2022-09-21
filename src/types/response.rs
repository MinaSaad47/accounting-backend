use std::borrow::Cow;

use rocket::{
    serde::{json::Json, Deserialize, Serialize},
    Responder,
};

use crate::accounting_api;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(crate = "rocket::serde")]
pub struct Response<T> {
    pub status: bool,
    pub message: String,
    pub data: Option<T>,
}

pub type ResponseResult<T> = Result<ResponseEnum<T>, ResponseEnum<T>>;

#[derive(Responder)]
pub enum ResponseEnum<T> {
    #[response(status = 200)]
    Ok(Json<Content<T>>),
    #[response(status = 201)]
    Created(Json<Content<T>>),
    #[response(status = 404)]
    NotFound(Json<Content<T>>),
    #[response(status = 204)]
    NoContent(Json<Content<T>>),
    #[response(status = 400)]
    Unauthorized(Json<Content<T>>),
    #[response(status = 501)]
    Internal(Json<Content<T>>),
}

impl<T> From<accounting_api::Error> for ResponseEnum<T> {
    fn from(error: accounting_api::Error) -> Self {
        match error {
            accounting_api::Error::ObjectNotFound => Self::not_found(format!("{error}").into()),
            _ => Self::internal(format!("{error}").into()),
        }
    }
}

impl<T> ResponseEnum<T> {
    pub fn ok(data: T, message: Cow<'static, str>) -> Self {
        ResponseEnum::Ok(Json(Content {
            status: true,
            message,
            data: Some(data),
        }))
    }
    pub fn created(data: T, message: Cow<'static, str>) -> Self {
        ResponseEnum::Created(Json(Content {
            status: true,
            message,
            data: Some(data),
        }))
    }
    pub fn not_found(message: Cow<'static, str>) -> Self {
        ResponseEnum::Created(Json(Content {
            status: false,
            message,
            data: None,
        }))
    }
    pub fn no_content(message: Cow<'static, str>) -> Self {
        ResponseEnum::Created(Json(Content {
            status: false,
            message,
            data: None,
        }))
    }
    pub fn unauthorized(message: Cow<'static, str>) -> Self {
        ResponseEnum::Created(Json(Content {
            status: false,
            message,
            data: None,
        }))
    }
    pub fn internal(message: Cow<'static, str>) -> Self {
        ResponseEnum::Created(Json(Content {
            status: false,
            message,
            data: None,
        }))
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(crate = "rocket::serde")]
pub struct Content<T> {
    pub status: bool,
    pub message: Cow<'static, str>,
    pub data: Option<T>,
}
