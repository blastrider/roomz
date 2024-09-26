use crate::schema::rooms;
use diesel::{Insertable, PgConnection, QueryResult, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "rooms"]
pub struct Room {
    pub id: Uuid,
    pub name: String,
    pub capacity: i32,
    pub equipments: Vec<String>,
    pub location: String,
}

impl Room {
    pub fn create(conn: &mut PgConnection, new_room: Room) -> QueryResult<Room> {
        diesel::insert_into(rooms::table)
            .values(&new_room)
            .get_result(conn)
    }
    pub fn list_all(conn: &mut PgConnection) -> QueryResult<Vec<Room>> {
        use crate::schema::rooms::dsl::*;

        rooms.load::<Room>(conn)
    }
}
