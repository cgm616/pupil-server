use std::env;
use std::io;
use std::path::{Path, PathBuf};
use std::ops::Deref;

use rocket::request;
use rocket::response::{Redirect, NamedFile};
use rocket::http::{Cookie, Cookies};
use rocket::State;
use rocket_contrib::{JSON, Value};

use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;

use super::model::{SafeUser, UserToken, Login, User, NewUser, Register};
use super::error::{Error, ThresholdKind};
use super::passwd;
use super::database::ConnectionPool;

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

#[get("/dash")]
fn dash(user: SafeUser) -> io::Result<NamedFile> {
    NamedFile::open("static/dash.html")
}

#[get("/dash")]
fn dash_redirect(cookies: &Cookies) -> Redirect {
    cookies.remove("jwt");
    Redirect::to("/login")
}

#[post("/login", format = "application/json", data = "<data>")]
fn login(cookies: &Cookies,
         data: JSON<Login>,
         pool: State<ConnectionPool>)
         -> Result<JSON<String>, Error> {
    use super::schema::users;

    let data = data.into_inner();

    let connection = pool.0.get()?;

    let user: User = users::table.filter(users::username.eq(&data.username))
        .first::<User>(connection.deref())?;

    if passwd::verify_password(user.pass.as_str(), data.password.as_str()) {
        if user.conf {
            let token = UserToken::new(user.clone())
                .construct_jwt(env::var("JWT_SECRET").expect("JWT_SECRET not set"));
            cookies.add(Cookie::new("jwt", token));
            Ok(JSON(String::from("dash")))
        } else {
            Err(Error::NotConfirmed(ThresholdKind::Login))
        }
    } else {
        Err(Error::BadUserOrPass)
    }

    // TODO make these errors the right errors
}

#[post("/register", format = "application/json", data = "<data>")]
fn register(cookies: &Cookies,
            data: JSON<Register>,
            pool: State<ConnectionPool>)
            -> Result<JSON<String>, Error> {
    use super::schema::users;

    let connection = pool.0.get()?;
    let data = data.into_inner();

    let secret = env::var("HASH_SECRET").expect("HASH_SECRET not set");
    let secure_pass = passwd::hash_password(data.username.as_str(),
                                            data.password.as_str(),
                                            secret.as_str());

    let new_user = NewUser {
        name: data.name.as_str(),
        email: data.email.as_str(),
        username: data.username.as_str(),
        pass: secure_pass.as_str(),
    };

    diesel::insert(&new_user).into(users::table)
        .execute(connection.deref())?;

    Err(Error::NotConfirmed(ThresholdKind::Register))

    // TODO: send confirmation email
}

#[get("/logout")]
fn logout(cookies: &Cookies) -> Redirect {
    cookies.remove("jwt");
    Redirect::to("/")
}

#[get("/favicon.ico")]
fn favicon() -> io::Result<NamedFile> {
    NamedFile::open("static/favicon.ico")
}

#[get("/static/<file..>")]
fn file(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("./static/static/").join(file)).ok()
}
