use std::env;
use std::collections::HashMap;

use rocket::request;
use rocket::response::Redirect;
use rocket::http::{Cookie, Cookies};
use rocket_contrib::Template;

use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;

use super::models::{SafeUser, Login, User, NewUser, Register, construct_token};

fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

#[get("/")]
fn index() -> Template {
    let mut context = HashMap::new();
    context.insert("name", "null");
    Template::render("index", &context)
}

#[get("/dash")]
fn dash(user: SafeUser) -> Template {
    let mut context = HashMap::new();
    context.insert("username", user.username);
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

#[post("/login", data="<form>")]
fn login(cookies: &Cookies, form: request::Form<Login>) -> Redirect {
    // TODO: validate password, get user from database, pass to `construct_token()`
    use super::schema::users;

    let connection = establish_connection();
    match users::table.filter(users::username.eq(form.get().username.clone()))
        .limit(1)
        .load::<User>(&connection) {
        Ok(user) => {
            if super::passwd::verify_password(user[0].pass.as_str(), form.get().password.as_str()) {
                cookies.add(Cookie::new("jwt", construct_token(user[0].clone())));
                Redirect::to("/dash")
            } else {
                Redirect::to("/login")
            }
        }
        Err(e) => return Redirect::to("/login"), // TODO: handle errors for real
    }
}

#[post("/register", data="<form>")]
fn register(cookies: &Cookies, form: request::Form<Register>) -> Redirect {
    use super::schema::users;
    use super::passwd::hash_password;

    let connection = establish_connection();
    let form = form.get(); // TODO: validation, make sure username doesn't exist

    let secret = env::var("HASH_SECRET").expect("HASH_SECRET not set");
    let secure_pass = hash_password(form.username.as_str(),
                                    form.password.as_str(),
                                    secret.as_str()); // TODO: hash password

    let new_user = NewUser {
        name: form.name.as_str(),
        email: form.email.as_str(),
        username: form.username.as_str(),
        pass: secure_pass.as_str(),
    };

    let user: User = diesel::insert(&new_user)
        .into(users::table)
        .get_result(&connection)
        .expect("Error saving new user");

    // TODO: send confirmation email
    // TODO: send better response that tells client to say something to user, error handling

    Redirect::to("/login")
}

#[get("/logout")]
fn logout(cookies: &Cookies) -> Redirect {
    cookies.remove("jwt");
    Redirect::to("/")
}
