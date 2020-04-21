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

#[derive(Deserialize, AsChangeset, Insertable)]
#[table_name = "heroes"]
pub struct InsertableHero {
    pub name: String,
    pub identity: String,
    pub hometown: String,
    pub age: i32
}

impl Hero {
    pub fn create(hero: InsertableHero, connection: HeroesDb) -> Hero {
        diesel::insert_into(heroes::table)
            .values(&hero)
            .execute(&connection.0)
            .expect("Error creating new hero");

        heroes::table.order(heroes::id.desc()).first(&connection.0).unwrap()
    }

    pub fn read(connection: HeroesDb) -> Vec<Hero> {
        heroes::table.order(heroes::id).load::<Hero>(&connection.0).unwrap()
    }

    pub fn update(id: i32, hero: InsertableHero, connection: HeroesDb) -> bool {
        diesel::update(heroes::table.find(id))
            .set(&hero)
            .execute(&connection.0)
            .is_ok()
    }

    pub fn delete(id: i32, connection: HeroesDb) -> bool {
        diesel::delete(heroes::table.find(id))
            .execute(&connection.0)
            .is_ok()
    }
}