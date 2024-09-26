// @generated automatically by Diesel CLI.

use diesel::{allow_tables_to_appear_in_same_query, joinable, table};

table! {
    rooms (id) {
        id -> Uuid,
        name -> Varchar,
        capacity -> Int4,
        equipments -> Array<Text>,
        location -> Varchar,
    }
}

table! {
    reservations (id) {
        id -> Uuid,
        room_id -> Uuid,
        user_id -> Uuid,
        start_time -> Timestamp,
        end_time -> Timestamp,
        priority -> Int4,
        status -> Varchar,
    }
}

table! {
    users (id) {
        id -> Uuid,
        name -> Varchar,
        role -> Varchar,
    }
}

joinable!(reservations -> rooms (room_id));
joinable!(reservations -> users (user_id));

allow_tables_to_appear_in_same_query!(rooms, reservations, users,);
