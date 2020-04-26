use diesel;
use diesel::prelude::*;
use crate::schema::hero;
use crate::HeroesDb;
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, Queryable)]
pub struct Hero {
    pub id: i32,
    pub name: String,
    pub identity: String,
    pub hometown: String,
    pub age: i32,
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime
}

#[derive(Deserialize, AsChangeset, Insertable)]
#[table_name = "hero"]
pub struct InsertableHero {
    pub name: String,
    pub identity: String,
    pub hometown: String,
    pub age: i32
}

impl Hero {
    pub fn create(hero: InsertableHero, connection: HeroesDb) -> Option<Hero> {
        diesel::insert_into(hero::table)
            .values(&hero)
            .get_result(&connection.0)
            .ok()
    }

    pub fn read_all(connection: HeroesDb) -> Vec<Hero> {
        hero::table
            .order(hero::id)
            .load::<Hero>(&connection.0)
            .expect("Error loading heroes")
    }

    pub fn read(id: i32, connection: HeroesDb) -> Option<Hero> {
        hero::table.find(id)
            .first(&connection.0)
            .ok()
    }

    pub fn update(id: i32, hero: InsertableHero, connection: HeroesDb) -> Option<Hero> {
        diesel::update(hero::table.find(id))
            .set(&hero)
            .get_result(&connection.0)
            .ok()
    }

    pub fn delete(id: i32, connection: HeroesDb) -> bool {
        diesel::delete(hero::table.find(id))
            .execute(&connection.0)
            .is_ok()
    }
}