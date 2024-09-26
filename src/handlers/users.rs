use crate::{db::DB_POOL, models::user::User};
use actix_web::{web, HttpResponse};

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
