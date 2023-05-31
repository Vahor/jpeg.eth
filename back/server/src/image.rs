use actix_web::{get, web, Error as AWError, HttpResponse};
use serde::Serialize;

use db::Pool;

use crate::db;

#[derive(Serialize)]
pub struct Image {
    pub image_id: i32,
    pub url: String,
}

#[get("/data/{nft_id}")]
pub async fn get_metadata(
    pool: web::Data<Pool>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AWError> {
    let result = db::get_image(&pool.get().unwrap(), *path);

    if result.is_err() {
        return Ok(HttpResponse::NotFound().finish());
    }

    Ok(HttpResponse::Ok().json(result.unwrap()))
}
