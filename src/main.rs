#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;


mod hero;
mod user;
mod cors;
mod auth;
mod jwt;

#[database("heroes")]
pub struct HeroesDb(diesel::PgConnection);

fn main() {
    let mut rocket = rocket::ignite()
        .attach(HeroesDb::fairing())
        .attach(cors::CorsFairing);
    rocket = hero::mount(rocket);
    rocket = user::mount(rocket);
    rocket.launch();
}