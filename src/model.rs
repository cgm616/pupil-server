use std::env;
use std::fmt;
use std::io;
use std::cmp::Ordering;

use rocket::request;
use rocket::outcome::Outcome;
use rocket::Request;
use rocket::http::{Cookie, Cookies, Status};

use jwt::{encode, decode, Header, Algorithm, Validation};
use jwt::errors::Error as JwtError;

use time;

use super::error::Error;

#[derive(Queryable, Clone, Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub username: String,
    pub pass: String,
    pub conf: bool,
}

#[derive(Debug, PartialEq)]
pub struct SafeUser {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub username: String,
    pub conf: bool,
}

use super::schema::users;

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub username: &'a str,
    pub pass: &'a str,
}

#[derive(Serialize, Deserialize)]
pub struct Login {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct Register {
    pub name: String,
    pub email: String,
    pub username: String,
    pub password: String,
}

impl<'a, 'r> request::FromRequest<'a, 'r> for SafeUser {
    type Error = Error;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<SafeUser, Error> {
        let cookies = request.cookies();
        let cookie = match cookies.find("jwt") {
            Some(cookie) => cookie,
            None => return Outcome::Failure((Status::NotFound, Error::BadCookie)),
        };

        let mut validation = Validation { iss: Some("pupil".to_string()), ..Default::default() };

        let secret = env::var("JWT_SECRET").expect("JWT_SECRET not set"); // TODO: better errors

        match decode::<UserToken>(&cookie.value(),
                                  secret.as_bytes(),
                                  Algorithm::HS256,
                                  &validation) {
            Ok(token) => Outcome::Success(SafeUser::from(token.claims)),
            Err(e) => {
                cookies.remove("jwt");
                Outcome::Failure((Status::NotFound, Error::BadCookie))
            }
        }
    }
}

impl From<User> for SafeUser {
    fn from(user: User) -> Self {
        SafeUser {
            id: user.id,
            name: user.name,
            email: user.email,
            username: user.username,
            conf: user.conf,
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
            conf: user.conf,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserToken {
    pub iat: i64,
    pub exp: i64,
    pub iss: String,
    pub id: i32,
    pub name: String,
    pub email: String,
    pub username: String,
    pub conf: bool,
}

static ONE_MIN: i64 = 60;
static ISSUER: &'static str = "pupil";

impl UserToken {
    pub fn new(user: User) -> Self {
        let now = time::get_time().sec;
        UserToken {
            iat: now,
            exp: now + ONE_MIN,
            iss: String::from(ISSUER),
            id: user.id,
            name: user.name,
            email: user.email,
            username: user.username,
            conf: user.conf,
        }
    }

    pub fn construct_jwt(&self, secret: String) -> String {
        encode(&Header::default(), self, secret.as_bytes()).unwrap() // TODO error handling
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_jwt() {
        let issued_at = 1492907635;
        let expired = 1492907695;
        let name = String::from("John Smith");
        let email = String::from("jsmith@website.com");
        let username = String::from("jsmith");
        let pass = String::from("hashed_password");
        let conf = true;

        let mut claims = UserToken::new(User {
            id: 1,
            name: name,
            email: email,
            username: username,
            pass: pass,
            conf: conf,
        });

        claims.iat = issued_at;
        claims.exp = expired;

        let encoded = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpYXQiOjE0OTI5MDc2MzUsImV4cCI6MTQ5Mj\
            kwNzY5NSwiaXNzIjoicHVwaWwiLCJpZCI6MSwibmFtZSI6IkpvaG4gU21pdGgiLCJlbWFpbCI6ImpzbWl0aEB3\
            ZWJzaXRlLmNvbSIsInVzZXJuYW1lIjoianNtaXRoIiwiY29uZiI6dHJ1ZX0.655jzhRXSF05RJyACAWv_tuT9p\
            MMVyIyMh4Icb6EKOI";

        assert_eq!(claims.construct_jwt(String::from("secret")), encoded);
    }
}
