#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use rocket_contrib::json::{Json, JsonValue};
use rocket::http::Status;

mod schema;
mod hero;
mod cors;
use crate::hero::{Hero, InsertableHero};

#[database("heroes")]
pub struct HeroesDb(diesel::PgConnection);

#[post("/", data = "<hero>")]
fn create(hero: Json<InsertableHero>, connection: HeroesDb) -> Json<Hero> {
    Json(Hero::create(hero.into_inner(), connection))
}

#[get("/")]
fn read_all(connection: HeroesDb) -> Json<Vec<Hero>> {
    Json(Hero::read_all(connection))
}


#[get("/<id>")]
fn read(id: i32, connection: HeroesDb) -> Result<Json<Hero>, Status> {
    match Hero::read(id, connection) {
        Some(hero) => Ok(Json(hero)),
        None => Err(Status::BadRequest)
    }
}

#[put("/<id>", data = "<hero>")]
fn update(id: i32, hero: Json<InsertableHero>, connection: HeroesDb) -> Json<JsonValue> {
    Json(json!({
        "success": Hero::update(id, hero.into_inner(), connection)
    }))
}

#[delete("/<id>")]
fn delete(id: i32, connection: HeroesDb) -> Json<JsonValue> {
    Json(json!({
        "success": Hero::delete(id, connection)
    }))
}

fn main() {
    rocket::ignite()
        .attach(HeroesDb::fairing())
        .attach(cors::CorsFairing)
        .mount("/heroes", routes![create, read, read_all, update, delete])
        .launch();
}