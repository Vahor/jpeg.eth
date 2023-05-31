use std::collections::{BTreeMap, HashMap};
use std::path::PathBuf;

use actix_web::{get, web, Error as AWError, HttpResponse};
use log::{debug, info, warn};
use serde::{Serialize, Serializer};

use db::Pool;

use crate::db;
use crate::db::register_image;

#[derive(Serialize)]
pub struct Image {
    pub image_id: String,
    #[serde(serialize_with = "ordered_map")]
    pub attributes: HashMap<String, String>,
}

fn ordered_map<S>(value: &HashMap<String, String>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
{
    let ordered: BTreeMap<_, _> = value.iter().collect();
    ordered.serialize(serializer)
}


#[get("/data")]
pub async fn get_all(
    pool: web::Data<Pool>,
) -> Result<HttpResponse, AWError> {
    let result = db::get_all_images(&pool.get().unwrap());

    if result.is_err() {
        let err = result.err().unwrap();
        debug!("Error: {:?}", err);
        return Ok(HttpResponse::NotFound().json("Not found"));
    }

    let result = result.unwrap();
    Ok(HttpResponse::Ok().json(result))
}

#[get("/data/{nft_id}")]
pub async fn get_metadata(
    pool: web::Data<Pool>,
    path: web::Path<String>,
) -> Result<HttpResponse, AWError> {
    let result = db::get_image(&pool.get().unwrap(), path.as_str());

    if result.is_err() {
        let err = result.err().unwrap();
        warn!("Error while querying image for token {}", path);
        debug!("Error: {:?}", err);
        return Ok(HttpResponse::NotFound().json("Not found"));
    }

    let result = result.unwrap();

    info!("result: {:?}", &result.attributes);

    Ok(HttpResponse::Ok().json(result))
}

pub fn load_images(pool: &Pool) {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let manifest_dir = PathBuf::from(manifest_dir);

    let images_dir = manifest_dir.join("resources/images");

    // Load all images in output folder, and insert in db if not already there
    let images = std::fs::read_dir(images_dir).unwrap();

    // get only .png files as .json have the same name
    let images = images.filter(|entry| {
        if let Ok(entry) = entry {
            if let Some(file_name) = entry.file_name().to_str() {
                return file_name.ends_with(".png");
            }
        }
        false
    });

    for image in images {
        if let Ok(image) = image {
            if let Some(file_name) = image.file_name().to_str() {
                let image_id = file_name.replace(".png", "");
                let attributes_path = image.path().with_extension("json");

                let attributes = std::fs::read_to_string(attributes_path).unwrap();

                register_image(&pool.get().unwrap(), image_id.parse().unwrap(), attributes)
                    .expect("Error inserting image in db");
            }
        }
    }
}
