use std::io;

use actix_web::{middleware, web, App, HttpServer};
use log::info;
use r2d2_sqlite::{self, SqliteConnectionManager};

use db::Pool;

use utils::env_helpers::{cast_required_env_var, set_default_env_var};
use crate::image::load_images;

mod db;
pub mod image;
mod listener;
mod image_routes;

#[actix_web::main]
async fn main() -> io::Result<()> {
    let resource_dir = cast_required_env_var::<String>("RESOURCE_DIR");

    set_default_env_var("RUST_LOG", "debug");
    env_logger::init();

    set_default_env_var("PORT", "8080");
    set_default_env_var("HOST", "127.0.0.1");

    let port = cast_required_env_var::<u16>("PORT");
    let host = cast_required_env_var::<String>("HOST");

    let manager = SqliteConnectionManager::file(format!("{}/app.db", resource_dir));

    let pool = Pool::new(manager).unwrap();

    // Init images
    load_images(&pool);

    info!("starting HTTP server at http://{}:{}", host, port);

    let listener = listener::start_listener(pool.clone());
    tokio::spawn(listener);

    HttpServer::new({
        let pool = pool.clone();
        move || {
            App::new()
                .wrap(middleware::Logger::default())
                .app_data(web::Data::new(pool.clone()))
                .service(image_routes::get_metadata)
                .service(image_routes::get_image)
                .service(image_routes::get_all)
        }
    })
    .bind((host, port))?
    .run()
    .await
}
