use std::path::PathBuf;

use fake::faker::lorem::en::{Paragraph, Word};
use fake::Fake;
use serde::{Deserialize, Serialize, Serializer};

use db::Pool;

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
