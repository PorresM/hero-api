#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use rocket_contrib::json::Json;
use rocket::http::Status;
use rocket::response::status;

mod schema;
mod hero;
mod cors;
use crate::hero::{Hero, InsertableHero};

#[database("heroes")]
pub struct HeroesDb(diesel::PgConnection);

#[post("/", format = "application/json", data = "<hero>")]
fn create(hero: Json<InsertableHero>, connection: HeroesDb) -> Result<status::Created<Json<Hero>>, Status> {
    match Hero::create(hero.into_inner(), connection) {
        Some(hero) => Ok(status::Created(format!("/heroes/{}", hero.id).to_string(), Some(Json(hero)))),
        None => Err(Status::BadRequest)
    }
}

#[get("/")]
fn read_all(connection: HeroesDb) -> Result<Json<Vec<Hero>>, Status> {
    Ok(Json(Hero::read_all(connection)))
}


#[get("/<id>")]
fn read(id: i32, connection: HeroesDb) -> Result<Json<Hero>, Status> {
    match Hero::read(id, connection) {
        Some(hero) => Ok(Json(hero)),
        None => Err(Status::NotFound)
    }
}

#[put("/<id>", format = "application/json", data = "<hero>")]
fn update(id: i32, hero: Json<InsertableHero>, connection: HeroesDb) -> Result<Json<Hero>, Status> {
    match Hero::update(id, hero.into_inner(), connection) {
        Some(hero) => Ok(Json(hero)),
        None => Err(Status::NotFound)
    }
}

#[delete("/<id>")]
fn delete(id: i32, connection: HeroesDb) -> Status {
    match Hero::delete(id, connection) {
        true => Status::NoContent,
        false => Status::InternalServerError
    }
}

fn main() {
    rocket::ignite()
        .attach(HeroesDb::fairing())
        .attach(cors::CorsFairing)
        .mount("/heroes", routes![create, read, read_all, update, delete])
        .launch();
}