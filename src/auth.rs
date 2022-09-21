use std::borrow::Cow;

use rocket::{
    http::Status,
    request::FromRequest,
    request::Outcome,
    serde::{Deserialize, Serialize},
    Request,
};

use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Claims {
    sub: i64,
    is_admin: bool,
    exp: usize,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ApiToken<'r>(pub Cow<'r, str>);

const SECRET: &str = "hello world secret";

impl<'r> ApiToken<'r> {
    pub fn generate(id: i64, is_admin: bool) -> Self {
        let claims = Claims { sub: id, is_admin, exp: usize::MAX };
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(SECRET.as_ref()),
        )
        .expect("valid encoded token");
        let api_token = Self(token.into());
        rocket::debug!("[token] generated: {api_token:?}");
        api_token
    }

    fn validate(&self) -> Option<(i64, bool)> {
        let token_data = decode::<Claims>(
            &self.0,
            &DecodingKey::from_secret(SECRET.as_ref()),
            &Validation::new(Algorithm::HS256),
        )
        .ok()?;

        Some((token_data.claims.sub, token_data.claims.is_admin))
    }
}

#[derive(Debug)]
pub enum ApiTokenError {
    Missing,
    Invalid,
}

pub struct UGuard(pub i64);
pub struct AGuard(pub i64);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AGuard {
    type Error = ApiTokenError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match request.headers().get_one("Authorization") {
            Some(auth) => {
                let api_token = ApiToken(auth.into());
                rocket::debug!("[token] validating: {api_token:?}");
                match api_token.validate() {
                    Some(t) => {
                        if t.1 {
                            Outcome::Success(AGuard(t.0))
                        } else {
                            Outcome::Forward(())
                        }
                    }
                    None => Outcome::Failure((Status::Unauthorized, ApiTokenError::Invalid)),
                }
            }
            None => Outcome::Failure((Status::BadRequest, ApiTokenError::Missing)),
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UGuard {
    type Error = ApiTokenError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match request.headers().get_one("Authorization") {
            Some(auth) => {
                let api_token = ApiToken(auth.into());
                rocket::debug!("[token] validating: {api_token:?}");
                match api_token.validate() {
                    Some(t) => {
                            Outcome::Success(UGuard(t.0))
                    }
                    None => Outcome::Failure((Status::Unauthorized, ApiTokenError::Invalid)),
                }
            }
            None => Outcome::Failure((Status::BadRequest, ApiTokenError::Missing)),
        }
    }
}
