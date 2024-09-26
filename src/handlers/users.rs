use crate::{db::DB_POOL, models::user::User};
use actix_web::{web, HttpResponse};
use uuid::Uuid;

pub async fn create_user(user_data: web::Json<User>) -> HttpResponse {
    let conn = &mut DB_POOL.get().unwrap();
    match User::create(conn, user_data.into_inner()) {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::BadRequest().json(err.to_string()),
    }
}

pub async fn list_users() -> HttpResponse {
    let conn = &mut DB_POOL.get().unwrap();
    match User::list_all(conn) {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn update_user(user_id: web::Path<Uuid>, user_data: web::Json<User>) -> HttpResponse {
    let conn = &mut DB_POOL.get().unwrap();
    match User::update(conn, *user_id, user_data.into_inner()) {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::BadRequest().json(err.to_string()),
    }
}

pub async fn delete_user(user_id: web::Path<Uuid>) -> HttpResponse {
    let conn = &mut DB_POOL.get().unwrap();
    match User::delete(conn, *user_id) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => HttpResponse::BadRequest().json(err.to_string()),
    }
}
