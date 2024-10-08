use crate::schema::rooms;
use chrono::NaiveDateTime;
use diesel::{
    BoolExpressionMethods, ExpressionMethods, Insertable, PgArrayExpressionMethods, PgConnection,
    QueryDsl, QueryResult, Queryable, RunQueryDsl,
};
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
    // Méthode pour rechercher les salles disponibles
    pub fn search_available_rooms(
        conn: &mut PgConnection,
        start_time: NaiveDateTime,
        end_time: NaiveDateTime,
        min_capacity: Option<i32>,
        required_equipments: Option<Vec<String>>,
    ) -> QueryResult<Vec<Room>> {
        use crate::schema::reservations::dsl::{
            end_time as res_end_time, reservations, room_id as res_room_id,
            start_time as res_start_time,
        };
        use crate::schema::rooms::dsl::*;

        let mut query = rooms.into_boxed();

        // Filtrer par capacité minimale
        if let Some(min_capacity_value) = min_capacity {
            query = query.filter(capacity.ge(min_capacity_value));
        }

        // Filtrer par équipements requis
        if let Some(required_equipments_value) = required_equipments {
            for equipment in required_equipments_value {
                query = query.filter(equipments.contains(vec![equipment]));
            }
        }

        // Exclure les salles qui sont réservées dans la période donnée
        let unavailable_rooms = reservations
            .filter(res_start_time.lt(end_time).and(res_end_time.gt(start_time)))
            .select(res_room_id);

        query
            .filter(id.ne_all(unavailable_rooms))
            .load::<Room>(conn)
    }
    pub fn is_available(
        conn: &mut PgConnection,
        room_id_value: Uuid,
        start_time_value: NaiveDateTime,
        end_time_value: NaiveDateTime,
    ) -> QueryResult<bool> {
        use crate::schema::reservations::dsl::{end_time, reservations, room_id, start_time};

        // Rechercher les réservations qui chevauchent la période donnée pour cette salle
        let count = reservations
            .filter(room_id.eq(room_id_value)) // Filtrer par `room_id`
            .filter(
                start_time
                    .lt(end_time_value) // La réservation commence avant la fin de la période demandée
                    .and(end_time.gt(start_time_value)), // La réservation se termine après le début de la période demandée
            )
            .count()
            .get_result::<i64>(conn)?;

        // Si `count` est 0, cela signifie qu'il n'y a pas de réservation chevauchante et donc la salle est disponible
        Ok(count == 0)
    }
    // Vérifie si une salle existe dans la base de données
    pub fn exists(conn: &mut PgConnection, room_id: Uuid) -> bool {
        use crate::schema::rooms::dsl::*;

        rooms
            .filter(id.eq(room_id))
            .select(id)
            .first::<Uuid>(conn)
            .is_ok()
    }
}
