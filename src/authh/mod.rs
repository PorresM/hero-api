
use rocket_contrib::json::Json;
use rocket::http::Status;
use rocket::response::status;

use crate::user::model::{User, InsertableUser};

use bcrypt::{verify};

use crate::HeroesDb;

#[post("/", format = "application/json", data = "<username, password>")]
fn login(username: String, password: String, connection: HeroesDb) -> Result<Json<User>, Status> {
    match User::find_by_username(username, connection) {
        Some(user) => {
            match verify("hunter2", &hashed) {
                Ok(valid) => valid,
                Err(_) => panic!()
            }
            Ok(Json(user))
        },
        None => Err(Status::NotFound)
    }
}


pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket.mount("/", routes![login])
}