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
extern crate r2d2;
extern crate r2d2_diesel;

use dotenv::dotenv;

mod schema;
mod models;
mod passwd;
mod server;
mod database;

use database::ConnectionPool;

fn main() {
    dotenv().ok();
    rocket::ignite()
        .manage(ConnectionPool::new())
        .mount("/",
               routes![server::index,
                       server::dash,
                       server::dash_redirect,
                       server::login,
                       server::register,
                       server::logout,
                       server::favicon,
                       server::file])
        .launch();
}
