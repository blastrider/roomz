use crate::db::DB_POOL;
use crate::models::Reservation;
use actix_web::{web, HttpResponse};

pub async fn create_reservation(reservation_data: web::Json<Reservation>) -> HttpResponse {
    let mut conn = DB_POOL.get().unwrap();
    match Reservation::create(&mut conn, reservation_data.into_inner()) {
        Ok(reservation) => HttpResponse::Ok().json(reservation),
        Err(err) => HttpResponse::BadRequest().json(err.to_string()),
    }
}

pub async fn get_reservation(reservation_id: web::Path<uuid::Uuid>) -> HttpResponse {
    let mut conn = DB_POOL.get().unwrap();
    match Reservation::get(&mut conn, *reservation_id) {
        Ok(reservation) => HttpResponse::Ok().json(reservation),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

pub async fn update_reservation(
    reservation_id: web::Path<uuid::Uuid>,
    reservation_data: web::Json<Reservation>,
) -> HttpResponse {
    let mut conn = DB_POOL.get().unwrap();
    match Reservation::update(&mut conn, *reservation_id, reservation_data.into_inner()) {
        Ok(reservation) => HttpResponse::Ok().json(reservation),
        Err(err) => HttpResponse::BadRequest().json(err.to_string()),
    }
}

pub async fn delete_reservation(reservation_id: web::Path<uuid::Uuid>) -> HttpResponse {
    let mut conn = DB_POOL.get().unwrap();
    match Reservation::delete(&mut conn, *reservation_id) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}

pub async fn list_reservations() -> HttpResponse {
    let conn = &mut DB_POOL.get().unwrap();
    match Reservation::list_all(conn) {
        Ok(reservations) => HttpResponse::Ok().json(reservations),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
