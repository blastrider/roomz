use crate::db::DB_POOL;
use crate::models::Room;
use actix_web::{web, HttpResponse};
use diesel::prelude::*;

pub async fn get_rooms() -> HttpResponse {
    use crate::schema::rooms::dsl::*;
    let mut conn = DB_POOL.get().unwrap();
    let results = rooms.load::<Room>(&mut conn).expect("Error loading rooms");
    HttpResponse::Ok().json(results)
}

pub async fn get_room(room_id: web::Path<uuid::Uuid>) -> HttpResponse {
    use crate::schema::rooms::dsl::*;
    let mut conn = DB_POOL.get().unwrap();
    let result = rooms.filter(id.eq(*room_id)).first::<Room>(&mut conn);
    match result {
        Ok(room) => HttpResponse::Ok().json(room),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

pub async fn get_availability(room_id: web::Path<uuid::Uuid>) -> HttpResponse {
    // Implémentation de la vérification de disponibilité
    HttpResponse::Ok().finish()
}

pub async fn create_room(room_data: web::Json<Room>) -> HttpResponse {
    let conn = &mut DB_POOL.get().unwrap();
    match Room::create(conn, room_data.into_inner()) {
        Ok(room) => HttpResponse::Ok().json(room),
        Err(err) => HttpResponse::BadRequest().json(err.to_string()),
    }
}

pub async fn list_rooms() -> HttpResponse {
    let conn = &mut DB_POOL.get().unwrap();
    match Room::list_all(conn) {
        Ok(rooms) => HttpResponse::Ok().json(rooms),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
