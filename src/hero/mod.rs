pub mod model;
pub mod schema;

use rocket_contrib::json::Json;
use rocket::http::Status;
use rocket::response::status;

use crate::HeroesDb;
use crate::hero::model::{Hero, InsertableHero};


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

pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket.mount("/heroes", routes![create, read, read_all, update, delete])
}