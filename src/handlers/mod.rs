pub mod reservations;
pub mod rooms;
pub mod users;

use actix_web::web;
use reservations::list_reservations;
use rooms::{create_room, list_rooms};
use users::{create_user, list_users};

pub fn rooms_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/rooms")
            .route("", web::get().to(rooms::get_rooms))
            .route("", web::get().to(list_rooms))
            .route("/{id}", web::get().to(rooms::get_room))
            .route("/{id}/availability", web::get().to(rooms::get_availability))
            .route("", web::post().to(create_room)),
    );
}

pub fn reservations_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/reservations")
            .route("", web::get().to(list_reservations))
            .route("", web::post().to(reservations::create_reservation))
            .route("/{id}", web::get().to(reservations::get_reservation))
            .route("/{id}", web::put().to(reservations::update_reservation))
            .route("/{id}", web::delete().to(reservations::delete_reservation)),
    );
}

pub fn users_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("", web::get().to(list_users)) // Route pour lister les utilisateurs
            .route("", web::post().to(create_user)), // Route pour créer un nouvel utilisateur
    );
}
