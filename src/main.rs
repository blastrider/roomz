use actix_web::{App, HttpServer};
use flexi_logger::{Duplicate, FileSpec, FlexiLoggerError, Logger, WriteMode};
use handlers::{reservations_config, rooms_config, users_config};

mod db;
mod errors;
mod handlers;
mod models;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Configure flexi_logger
    // Configure flexi_logger
    let log_file_name = format!("roomz_{}.log", chrono::Local::now().format("%Y-%m-%d"));
    Logger::try_with_str("info")
        .map_err(|e: FlexiLoggerError| {
            std::io::Error::new(std::io::ErrorKind::Other, e.to_string())
        })?
        .log_to_file(FileSpec::default().basename(log_file_name))
        .duplicate_to_stdout(Duplicate::Info)
        .write_mode(WriteMode::BufferAndFlush)
        .append()
        .format_for_files(flexi_logger::detailed_format)
        .use_utc()
        .rotate(
            flexi_logger::Criterion::Size(10_000_000), // Rotate when file size exceeds 10 MB
            flexi_logger::Naming::Numbers,
            flexi_logger::Cleanup::KeepLogFiles(7), // Keep the last 7 log files
        )
        .start()
        .map_err(|e: FlexiLoggerError| {
            std::io::Error::new(std::io::ErrorKind::Other, e.to_string())
        })?;

    // Application initialization

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
