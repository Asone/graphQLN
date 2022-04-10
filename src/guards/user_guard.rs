use std::env;

extern crate dotenv;

use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    Request,
};

use crate::db::models::user_token::UserToken;

pub struct UserGuard(pub bool);

impl UserGuard {
    /// Checks that the authorization header includes Bearer mention
    /// Returns the token without the bearer prefix
    pub fn format_bearer(authorization: &str) -> String {
        let re = regex::Regex::new("^[bB]earer ").unwrap();
        re.replace(authorization, "").to_string()
    }

    fn get_secret() -> Result<String, ()> {
        let secret = env::var("JWT_TOKEN_SECRET");

        match secret {
            Ok(secret) => Ok(secret),
            Err(_) => Err(()),
        }
    }
}

/// Checks the JWT provided and checks if it is valid
#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserGuard {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let authorization = request.headers().get_one("Authorization");

        match authorization {
            Some(authorization) => {
                let formated_token = Self::format_bearer(authorization);
                let secret = Self::get_secret().unwrap();
                let token = jsonwebtoken::decode::<UserToken>(
                    formated_token.as_str(),
                    &DecodingKey::from_secret(secret.as_ref()),
                    &Validation::new(Algorithm::HS256),
                );

                match token {
                    Ok(token) => {
                        let t = token.claims;
                        Outcome::Success(UserGuard(true))
                    }
                    Err(_) => Outcome::Failure((Status::ExpectationFailed, ())),
                }
            }
            None => Outcome::Failure((Status::Forbidden, ())),
        }
    }
}
