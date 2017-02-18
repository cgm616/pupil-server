#[derive(Queryable)]
#[derive(Clone)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub username: String,
    pub pass: String,
    pub conf: bool,
}

pub struct SafeUser {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub username: String,
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct UserToken {
    pub iat: i64,
    pub exp: i64,
    pub id: i32,
    pub name: String,
    pub email: String,
    pub username: String,
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

#[derive(FromForm)]
pub struct Login {
    pub username: String,
    pub password: String,
}

#[derive(FromForm)]
pub struct Register {
    pub name: String,
    pub email: String,
    pub username: String,
    pub password: String,
}

use std::env;
use std::cmp::Ordering;

use rocket::request;
use rocket::outcome::Outcome;
use rocket::Request;
use rocket::http::{Cookie, Cookies};

use jwt::{encode, decode, Header, Algorithm};
use jwt::errors::Error;

use time;

impl<'a, 'r> request::FromRequest<'a, 'r> for SafeUser {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<SafeUser, ()> {
        let cookies = request.cookies();
        let cookie = match cookies.find("jwt") {
            Some(cookie) => cookie,
            None => return Outcome::Forward(()),
        };

        let secret = env::var("JWT_SECRET").expect("JWT_SECRET not set"); // TODO: better errors
        let token_data =
            match decode::<UserToken>(&cookie.value(), secret.as_bytes(), Algorithm::HS256) {
                Ok(token) => token,
                Err(e) => return Outcome::Forward(()),
            };

        match token_data.claims.iat.cmp(&token_data.claims.exp) {
            Ordering::Less => {
                match token_data.claims.exp.cmp(&time::get_time().sec) {
                    Ordering::Less | Ordering::Equal => return Outcome::Forward(()),
                    _ => {}
                }
            }
            _ => return Outcome::Forward(()),
        }

        Outcome::Success(SafeUser {
            id: token_data.claims.id,
            name: token_data.claims.name,
            email: token_data.claims.email,
            username: token_data.claims.username,
        })
    }
}

static ONE_MIN: i64 = 60;

pub fn construct_token(user: User) -> String {
    let now = time::get_time().sec;
    let payload = UserToken {
        iat: now,
        exp: now + ONE_MIN,
        id: user.id,
        name: user.name,
        email: user.email,
        username: user.username,
    };

    let secret = env::var("JWT_SECRET").expect("JWT_SECRET not set");
    encode(Header::default(), &payload, secret.as_bytes()).unwrap()
}
