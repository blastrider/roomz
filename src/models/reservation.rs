use crate::schema::reservations;
use chrono::NaiveDateTime;
use diesel::{
    ExpressionMethods, Insertable, PgConnection, QueryDsl, QueryResult, Queryable, RunQueryDsl,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "reservations"]
pub struct Reservation {
    pub id: Uuid,
    pub room_id: Uuid,
    pub user_id: Uuid,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub priority: i32,
    pub status: String,
}

impl Reservation {
    pub fn create(
        conn: &mut PgConnection,
        new_reservation: Reservation,
    ) -> QueryResult<Reservation> {
        diesel::insert_into(reservations::table)
            .values(&new_reservation)
            .get_result(conn)
    }
    pub fn get(conn: &mut PgConnection, reservation_id: Uuid) -> QueryResult<Reservation> {
        use crate::schema::reservations::dsl::*;

        reservations
            .filter(id.eq(reservation_id))
            .first::<Reservation>(conn)
    }
    pub fn update(
        conn: &mut PgConnection,
        reservation_id: Uuid,
        updated_reservation: Reservation,
    ) -> QueryResult<Reservation> {
        use crate::schema::reservations::dsl::*;

        diesel::update(reservations.filter(id.eq(reservation_id)))
            .set((
                room_id.eq(updated_reservation.room_id),
                user_id.eq(updated_reservation.user_id),
                start_time.eq(updated_reservation.start_time),
                end_time.eq(updated_reservation.end_time),
                priority.eq(updated_reservation.priority),
                status.eq(updated_reservation.status),
            ))
            .get_result(conn)
    }
    pub fn delete(conn: &mut PgConnection, reservation_id: Uuid) -> QueryResult<usize> {
        use crate::schema::reservations::dsl::*;

        diesel::delete(reservations.filter(id.eq(reservation_id))).execute(conn)
    }
    pub fn list_all(conn: &mut PgConnection) -> QueryResult<Vec<Reservation>> {
        use crate::schema::reservations::dsl::*;

        reservations.load::<Reservation>(conn)
    }
}
