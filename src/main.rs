use actix_web::{App, HttpServer};
use handlers::{reservations_config, rooms_config, users_config};

mod db;
mod errors;
mod handlers;
mod models;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    db::init();

    HttpServer::new(|| {
        App::new()
            .configure(rooms_config)
            .configure(reservations_config)
            .configure(users_config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
