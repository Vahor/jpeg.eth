use std::fs;
use std::path::PathBuf;

use image::DynamicImage;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ArtLayerMeta {
    /// Offset from the top left corner of the image
    pub x_offset: i64,
    /// Offset from the top left corner of the image
    pub y_offset: i64,
}

impl ArtLayerMeta {
    fn from_json(json: &str) -> ArtLayerMeta {
        let meta_data: ArtLayerMeta = match serde_json::from_str(json) {
            Ok(data) => data,
            Err(error) => panic!("Error parsing meta file: {}", error),
        };
        meta_data
    }

    fn from_file(file: &str) -> ArtLayerMeta {
        let meta_data = match fs::read_to_string(file) {
            Ok(data) => data,
            Err(error) => panic!("Error reading meta file: {}", error),
        };
        return ArtLayerMeta::from_json(&meta_data);
    }
}

pub struct ArtLayerElement {
    /// Name of the element (file name with extension)
    pub name: String,
    /// Type of the element
    pub element_type: String,
    /// The image of the element
    pub image: DynamicImage,
}

pub struct ArtLayer {
    /// Name of the layer (folder name)
    pub name: String,
    /// The meta data of the collection
    pub meta: ArtLayerMeta,
    /// The elements of the collection
    pub elements: Vec<ArtLayerElement>,
}

impl ArtLayer {
    pub fn from_folder(folder: PathBuf) -> ArtLayer {
        let folder_name = folder
            .file_name()
            .expect("Error reading folder name")
            .to_str()
            .expect("Error parsing folder name");

        // Read the _meta.json file in the current folder
        let meta_file = folder.join("_meta.json");
        let meta_data =
            ArtLayerMeta::from_file(meta_file.to_str().expect("Error reading meta file"));

        // Read all .png files in the current folder
        let png_files = match fs::read_dir(&folder) {
            Ok(entries) => entries.filter(|entry| {
                if let Ok(entry) = entry {
                    if let Some(file_name) = entry.file_name().to_str() {
                        return file_name.ends_with(".png");
                    }
                }
                false
            }),
            Err(error) => panic!("Error reading folder: {}", error),
        };

        // Load all images
        let mut images = Vec::new();
        for png_file in png_files {
            if let Ok(png_file) = png_file {
                let image = match image::open(png_file.path()) {
                    Ok(image) => image,
                    Err(error) => panic!("Error reading image: {}", error),
                };
                let name = png_file
                    .file_name()
                    .into_string()
                    .expect("Error reading file name");
                images.push((name.to_string(), image));
            }
        }

        // Create the collection
        let layer = ArtLayer {
            name: folder_name.to_string(),
            meta: meta_data,
            elements: images
                .into_iter()
                .map(|image| ArtLayerElement {
                    name: image.0,
                    image: image.1,
                    element_type: folder_name.to_string(),
                })
                .collect(),
        };

        layer
    }
}
