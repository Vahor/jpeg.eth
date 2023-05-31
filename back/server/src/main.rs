use std::io;

use actix_web::{web, App, HttpServer, middleware};
use r2d2_sqlite::{self, SqliteConnectionManager};

use db::Pool;
use crate::env_helpers::{cast_required_env_var, set_default_env_var};

pub mod alchemy;
mod db;
pub mod image;
mod env_helpers;

#[actix_web::main]
async fn main() -> io::Result<()> {
    set_default_env_var("RUST_LOG", "info");
    env_logger::init();

    set_default_env_var("PORT", "8080");
    set_default_env_var("HOST", "127.0.0.1");
    set_default_env_var("SIGNING_KEY", "whsec_test");

    let port = cast_required_env_var::<u16>("PORT");
    let host = cast_required_env_var::<String>("HOST");

    let manager = SqliteConnectionManager::file("resources/app.db");

    let pool = Pool::new(manager).unwrap();

    println!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .service(image::get_metadata)
            .service(alchemy::webhook)
    })
    .bind((host, port))?
    .run()
    .await
}
