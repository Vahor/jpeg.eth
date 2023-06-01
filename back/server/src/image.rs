use fake::faker::lorem::en::{Paragraph, Word};
use fake::Fake;
use log::info;
use serde::{Deserialize, Serialize, Serializer};

use db::Pool;
use utils::env_helpers::cast_required_env_var;

use crate::db;
use crate::db::register_image;

#[derive(Serialize)]
pub struct Image {
    pub image_id: String,
    pub attributes: Vec<Attribute>,
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Attribute {
    pub trait_type: String,
    pub value: String,
}

#[derive(Serialize, Debug)]
pub struct ImageMetadata {
    pub description: String,
    pub external_url: String,
    pub image: String,
    pub name: String,
    #[serde(serialize_with = "ordered_list")]
    pub attributes: Vec<Attribute>,
}

fn ordered_list<S>(value: &Vec<Attribute>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut ordered = vec![];
    for attribute in value {
        ordered.push(attribute);
    }
    ordered.sort_by(|a, b| a.trait_type.cmp(&b.trait_type));
    return ordered.serialize(serializer);
}

pub fn load_images(pool: &Pool) {
    let resource_dir = cast_required_env_var::<String>("RESOURCE_DIR");

    let images_dir = format!("{}/output", resource_dir);

    info!("Loading images from {}", images_dir);

    // Load all images in output folder, and insert in db if not already there
    let images = std::fs::read_dir(images_dir).unwrap();

    for image in images {
        if let Ok(image) = image {
            if let Some(file_name) = image.file_name().to_str() {
                // Skip .json files
                if !file_name.ends_with(".png") {
                    continue;
                }

                let image_id = file_name.replace(".png", "");
                let attributes_path = image.path().with_extension("json");

                let attributes = std::fs::read_to_string(attributes_path).unwrap();

                let fake_name = Word().fake();
                let fake_description = Paragraph(2..5).fake();

                register_image(
                    &pool.get().unwrap(),
                    image_id.parse().unwrap(),
                    attributes,
                    fake_name,
                    fake_description,
                )
                .expect("Error inserting image in db");
            }
        }
    }
}
