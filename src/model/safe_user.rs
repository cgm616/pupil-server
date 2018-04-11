use std::env;

use rocket::request::{self, Request};
use rocket::http::{Cookie, Status};
use rocket::Outcome;
use jwt::{decode, Validation};
use failure::Fail;

use super::user::User;
use super::token::UserToken;
use fail::{PupilError, PupilErrorKind};

/// A user data struct safe for memory usage (no password hash).
/// Implements `FromRequest` for use as a request guard.
#[derive(Debug, PartialEq)]
pub struct SafeUser {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub username: String,
}

impl From<User> for SafeUser {
    fn from(user: User) -> Self {
        SafeUser {
            id: user.id,
            name: user.name,
            email: user.email,
            username: user.username,
        }
    }
}

impl From<UserToken> for SafeUser {
    fn from(user: UserToken) -> Self {
        SafeUser {
            id: user.id,
            name: user.name,
            email: user.email,
            username: user.username,
        }
    }
}

impl<'a, 'r> request::FromRequest<'a, 'r> for SafeUser {
    type Error = PupilError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<SafeUser, Self::Error> {
        let mut cookies = request.cookies();

        let cookie = match cookies.get("jwt") {
            Some(cookie) => cookie.to_owned(),
            None => return Outcome::Failure((Status::NotFound, PupilErrorKind::BadCookie.into())),
        };

        let validation = Validation {
            iss: Some("pupil".to_string()),
            ..Default::default()
        };

        let secret = match env::var("JWT_SECRET") {
            Ok(v) => v,
            Err(err) => {
                return Outcome::Failure((
                    Status::InternalServerError,
                    err.context(PupilErrorKind::EnvVar).into(),
                ))
            }
        }; // TODO: better errors

        match decode::<UserToken>(cookie.value(), secret.as_bytes(), &validation) {
            Ok(token) => Outcome::Success(SafeUser::from(token.claims)),
            Err(_) => {
                cookies.remove(Cookie::new("jwt", "invalidtoken"));
                Outcome::Failure((Status::NotFound, PupilErrorKind::BadCookie.into()))
            }
        }
    }
}
