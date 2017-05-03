#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(custom_derive)]
//#![feature(proc_macro)]

extern crate dotenv;
extern crate rocket_contrib;
extern crate rocket;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate jsonwebtoken as jwt;
extern crate time;
extern crate argon2rs;
extern crate rand;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate r2d2;
extern crate r2d2_diesel;

use dotenv::dotenv;

mod schema;
mod model;
mod passwd;
mod server;
mod database;
mod error;

use database::ConnectionPool;

fn main() {
    dotenv().ok();
    rocket::ignite()
        .manage(ConnectionPool::new())
        .mount("/",
               routes![server::index,
                       server::dash,
                       server::login,
                       server::register,
                       server::logout,
                       server::favicon,
                       server::file])
        .launch();
}
