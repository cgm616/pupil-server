#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(custom_derive)]
//#![feature(proc_macro)]

extern crate dotenv;
extern crate rocket_contrib;
extern crate rocket;
extern crate jsonwebtoken as jwt;
extern crate rustc_serialize;
extern crate time;
extern crate argon2rs;
extern crate rand;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;

use dotenv::dotenv;

mod schema;
mod models;
mod passwd;
mod server;

fn main() {
    dotenv().ok();
    rocket::ignite()
        .mount("/",
               routes![server::index,
                       server::dash,
                       server::dash_redirect,
                       server::threshold,
                       server::login,
                       server::register,
                       server::logout])
        .launch();
}
