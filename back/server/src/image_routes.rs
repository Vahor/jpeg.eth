use std::path::PathBuf;

use actix_files::NamedFile;
use actix_web::http::header::{ContentDisposition, DispositionType};
use actix_web::{get, web, Error as AWError, HttpRequest, HttpResponse};
use log::{debug, warn};

use db::Pool;

use crate::db;
use crate::env_helpers::cast_required_env_var;
use crate::image::ImageMetadata;

#[get("/data")]
pub async fn get_all(pool: web::Data<Pool>) -> Result<HttpResponse, AWError> {
    let result = db::get_all_images(&pool.get().unwrap());

    if result.is_err() {
        let err = result.err().unwrap();
        debug!("Error: {:?}", err);
        return Ok(HttpResponse::NotFound().json("Not found"));
    }

    let result = result.unwrap();
    Ok(HttpResponse::Ok().json(result))
}

#[get("/data/{token_id}")]
pub async fn get_metadata(
    pool: web::Data<Pool>,
    token_id: web::Path<String>,
) -> Result<HttpResponse, AWError> {
    let result = db::get_image(&pool.get().unwrap(), token_id.as_str());

    if result.is_err() {
        let err = result.err().unwrap();
        debug!("Error: {:?}", err);
        return Ok(HttpResponse::NotFound().json("Not found"));
    }

    let domain = cast_required_env_var::<String>("DOMAIN");

    let result = result.unwrap();
    let metadata = ImageMetadata {
        name: result.name,
        description: result.description,
        external_url: format!("{}/data/{}", domain, token_id),
        image: format!("{}/image/{}", domain, token_id),
        attributes: result.attributes,
    };

    Ok(HttpResponse::Ok().json(metadata))
}

#[get("/image/{token_id}")]
pub async fn get_image(
    pool: web::Data<Pool>,
    token_id: web::Path<String>,
    req: HttpRequest,
) -> Result<HttpResponse, AWError> {
    let result = db::get_image(&pool.get().unwrap(), token_id.as_str());

    if result.is_err() {
        let err = result.err().unwrap();
        warn!("Error while querying image for token {}", token_id);
        debug!("Error: {:?}", err);
        return Ok(HttpResponse::NotFound().json("Not found"));
    }

    let result = result.unwrap();

    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let manifest_dir = PathBuf::from(manifest_dir);

    let image_path = format!("resources/images/{}.png", result.image_id);
    let image_path = manifest_dir.join(image_path);

    let file = NamedFile::open(image_path)?;

    Ok(file
        .set_content_disposition(ContentDisposition {
            disposition: DispositionType::Inline,
            parameters: vec![],
        })
        .into_response(&req))
}
