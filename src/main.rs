mod config;
mod handler;
mod models;
mod routes;

use actix_web::{App, HttpServer};
use config::AppConfig;
use routes::todo_route::configure_todo_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = AppConfig::new();

    HttpServer::new(move || App::new().configure(configure_todo_routes))
        .bind(format!("{}:{}", config.host, config.port))?
        .run()
        .await
}
