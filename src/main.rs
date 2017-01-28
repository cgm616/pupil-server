#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(custom_derive)]

extern crate rocket_contrib;
extern crate rocket;
extern crate jsonwebtoken as jwt;
extern crate rustc_serialize;
extern crate time;

use std::collections::HashMap;
use std::cmp::Ordering;

use rocket::request;
use rocket::outcome::Outcome;
use rocket::Request;
use rocket::response::Redirect;
use rocket::http::{Cookie, Cookies, Status};
use rocket_contrib::Template;

use jwt::{encode, decode, Header, Algorithm};
use jwt::errors::{Error};

#[derive(Debug, RustcEncodable, RustcDecodable)]
struct UserToken {
    iat: i64,
    exp: i64,
    user: String,
    roles: Vec<String>,
}

#[derive(FromForm)]
struct Login {
    username: String,
    password: String,
}

#[derive(FromForm)]
struct Register {
    username: String,
    email: String,
    first_name: String,
    password: String,
}

struct User {
    name: String,
    roles: Vec<String>,
}

impl<'a, 'r> request::FromRequest<'a, 'r> for User {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<User, ()> {
        let cookies = request.cookies();
        let cookie = match cookies.find("jwt") {
            Some(cookie) => cookie,
            None => return Outcome::Forward(()),
        };

        let token_data = match decode::<UserToken>(&cookie.value, "secret".as_bytes(), Algorithm::HS256) {
            Ok(token) => token,
            Err(e) => return Outcome::Forward(()),
        };

        match token_data.claims.iat.cmp(&token_data.claims.exp) {
            Ordering::Less => {
                match token_data.claims.exp.cmp(&time::get_time().sec) {
                    Ordering::Less | Ordering::Equal => return Outcome::Forward(()),
                    _ => {},
                }
            },
            _ => return Outcome::Forward(()),
        }
        // TODO: check expiration and issuance date...

        Outcome::Success(User {
            name: token_data.claims.user,
            roles: token_data.claims.roles,
        })
    }
}

static ONE_MIN: i64 = 60;

fn construct_token(user: User) -> String {
    let now = time::get_time().sec;
    let payload = UserToken {
        iat: now,
        exp: now + ONE_MIN,
        user: user.name,
        roles: user.roles,
    };

    encode(Header::default(), &payload, "secret".as_bytes()).unwrap()
}

#[get("/")]
fn index() -> Template {
    let mut context = HashMap::new();
    context.insert("name", "null");
    Template::render("index", &context)
}

#[get("/dash")]
fn dash(user: User) -> Template {
    let mut context = HashMap::new();
    context.insert("name", user.name);
    context.insert("role", user.roles[0].clone());
    Template::render("dash", &context)
}

#[get("/dash")]
fn dash_redirect(cookies: &Cookies) -> Redirect {
    cookies.remove("jwt");
    Redirect::to("/login")
}

#[get("/login")]
fn threshold() -> Template {
    let mut context = HashMap::new();
    context.insert("name", "null");
    Template::render("threshold", &context)
}

#[post("/login", data="<login_form>")]
fn login(cookies: &Cookies, login_form: request::Form<Login>) -> Redirect {
    let user = User {
        name: login_form.get().username.clone(),
        roles: vec!["admin".to_string(), "user".to_string()],
    };

    cookies.add(Cookie::new("jwt".into(), construct_token(user)));
    Redirect::to("/dash")
}

#[get("/logout")]
fn logout(cookies: &Cookies) -> Redirect {
    cookies.remove("jwt");
    Redirect::to("/")
}

fn main() {
    rocket::ignite().mount("/", routes![index, dash, dash_redirect, threshold, login, logout]).launch();
}
