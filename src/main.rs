#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(custom_derive)]
#![feature(proc_macro)]
#![feature(use_nested_groups)]
#![feature(decl_macro)]

extern crate argon2rs;
extern crate chrono;
extern crate crockford;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
extern crate dotenv;
extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate jsonwebtoken as jwt;
extern crate lettre;
extern crate psl;
extern crate publicsuffix;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rand;
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use std::env;
use std::sync::{Arc, Mutex};

use dotenv::dotenv;

use publicsuffix::LIST_URL;

use database::new_pool;

use lettre::{smtp, SmtpTransport};

use failure::ResultExt;

mod schema;
mod model;
mod passwd;
mod route;
mod database;
mod fail;

use fail::{PupilError, PupilErrorKind};

type Mailer = Arc<Mutex<SmtpTransport>>;

embed_migrations!();

fn main() {
    run().unwrap_or_else(|err| println!("The server exited with error: {:?}", err));
}

pub fn init_rocket() -> Result<rocket::Rocket, PupilError> {
    Ok(rocket::ignite()
        .manage(new_pool()?)
        .manage(Arc::new(Mutex::new(
            SmtpTransport::simple_builder("smtp.fastmail.com".to_string())
                .context(PupilErrorKind::EmailError)?
                .hello_name(smtp::extension::ClientId::new("usepupil.us".to_string()))
                .credentials(smtp::authentication::Credentials::new(
                    env::var("MAIL_USERNAME").context(PupilErrorKind::EnvVar)?,
                    env::var("MAIL_PASSWORD").context(PupilErrorKind::EnvVar)?,
                ))
                .smtp_utf8(true)
                .authentication_mechanism(smtp::authentication::Mechanism::Plain)
                .connection_reuse(smtp::ConnectionReuseParameters::ReuseUnlimited)
                .build(),
        )))
        .mount(
            "/",
            routes![
                route::index,
                route::dash,
                route::login,
                route::register,
                route::logout,
                route::favicon,
                route::file,
                route::not_confirmed,
                route::confirm,
            ],
        ))
}

fn run() -> Result<(), PupilError> {
    dotenv().ok();

    psl::init(LIST_URL, None, None).unwrap();

    init_rocket()?.launch();

    Ok(())
}
