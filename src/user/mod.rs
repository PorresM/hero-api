pub mod model;
pub mod schema;

use rocket_contrib::json::Json;
use rocket::http::Status;
use rocket::response::status;

use crate::HeroesDb;
use crate::user::model::{User, InsertableUser};


#[post("/", format = "application/json", data = "<user>")]
fn create(user: Json<InsertableUser>, connection: HeroesDb) -> Result<status::Created<Json<User>>, Status> {
    match User::create(user.into_inner(), connection) {
        Some(user) => Ok(status::Created(format!("/users/{}", user.id).to_string(), Some(Json(user)))),
        None => Err(Status::BadRequest)
    }
}

#[get("/")]
fn read_all(connection: HeroesDb) -> Result<Json<Vec<User>>, Status> {
    Ok(Json(User::read_all(connection)))
}


#[get("/<id>")]
fn read(id: i32, connection: HeroesDb) -> Result<Json<User>, Status> {
    match User::read(id, connection) {
        Some(user) => Ok(Json(user)),
        None => Err(Status::NotFound)
    }
}

#[put("/<id>", format = "application/json", data = "<user>")]
fn update(id: i32, user: Json<InsertableUser>, connection: HeroesDb) -> Result<Json<User>, Status> {
    match User::update(id, user.into_inner(), connection) {
        Some(user) => Ok(Json(user)),
        None => Err(Status::NotFound)
    }
}

#[delete("/<id>")]
fn delete(id: i32, connection: HeroesDb) -> Status {
    match User::delete(id, connection) {
        true => Status::NoContent,
        false => Status::InternalServerError
    }
}

pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket.mount("/users", routes![create, read, read_all, update, delete])
}