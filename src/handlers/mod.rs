pub mod reservations;
pub mod rooms;
pub mod users;

use actix_web::web;
use reservations::list_reservations;
use rooms::{create_room, list_rooms, search_rooms};
use users::{create_user, delete_user, list_users, update_user};

pub fn rooms_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/rooms")
            .route("/all", web::get().to(rooms::get_rooms))
            .route("/list", web::get().to(list_rooms))
            .route("/{id}", web::get().to(rooms::get_room))
            .route("/{id}/availability", web::get().to(rooms::get_availability))
            .route("/create", web::post().to(create_room))
            .route("/search", web::get().to(search_rooms)),
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
            .route("", web::post().to(create_user)) // Route pour créer un utilisateur
            .route("/{id}", web::put().to(update_user)) // Route pour mettre à jour un utilisateur
            .route("/{id}", web::delete().to(delete_user)), // Route pour supprimer un utilisateur
    );
}
