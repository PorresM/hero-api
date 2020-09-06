use diesel;
use diesel::prelude::*;
use crate::HeroesDb;
use crate::user::schema::user;
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub roles: Vec<String>,
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime
}

#[derive(Deserialize, AsChangeset, Insertable)]
#[table_name = "user"]
pub struct InsertableUser {
    pub username: String,
    pub password: String,
    pub roles: Vec<String>
}

impl User {
    pub fn create(user: InsertableUser, connection: HeroesDb) -> Option<User> {
        diesel::insert_into(user::table)
            .values(&user)
            .get_result(&connection.0)
            .ok()
    }

    pub fn read_all(connection: HeroesDb) -> Vec<User> {
        user::table
            .order(user::id)
            .load::<User>(&connection.0)
            .expect("Error loading users")
    }

    pub fn read(id: i32, connection: HeroesDb) -> Option<User> {
        user::table.find(id)
            .first(&connection.0)
            .ok()
    }

    pub fn update(id: i32, user: InsertableUser, connection: HeroesDb) -> Option<User> {
        diesel::update(user::table.find(id))
            .set(&user)
            .get_result(&connection.0)
            .ok()
    }

    pub fn delete(id: i32, connection: HeroesDb) -> bool {
        diesel::delete(user::table.find(id))
            .execute(&connection.0)
            .is_ok()
    }
}