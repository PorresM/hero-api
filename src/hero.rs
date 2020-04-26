use diesel;
use diesel::prelude::*;
use crate::schema::heroes;
use crate::HeroesDb;

#[derive(Serialize, Deserialize, Queryable)]
pub struct Hero {
    pub id: i32,
    pub name: String,
    pub identity: String,
    pub hometown: String,
    pub age: i32
}
// TODO: add created_at, updated_at

#[derive(Deserialize, AsChangeset, Insertable)]
#[table_name = "heroes"]
pub struct InsertableHero {
    pub name: String,
    pub identity: String,
    pub hometown: String,
    pub age: i32
}

impl Hero {
    pub fn create(hero: InsertableHero, connection: HeroesDb) -> Option<Hero> {
        diesel::insert_into(heroes::table)
            .values(&hero)
            .get_result(&connection.0)
            .ok()
    }

    pub fn read_all(connection: HeroesDb) -> Vec<Hero> {
        heroes::table
            .order(heroes::id)
            .load::<Hero>(&connection.0)
            .expect("Error loading heroes")
    }

    pub fn read(id: i32, connection: HeroesDb) -> Option<Hero> {
        heroes::table.find(id)
            .first(&connection.0)
            .ok()
    }

    pub fn update(id: i32, hero: InsertableHero, connection: HeroesDb) -> Option<Hero> {
        diesel::update(heroes::table.find(id))
            .set(&hero)
            .get_result(&connection.0)
            .ok()
    }

    pub fn delete(id: i32, connection: HeroesDb) -> bool {
        diesel::delete(heroes::table.find(id))
            .execute(&connection.0)
            .is_ok()
    }
}