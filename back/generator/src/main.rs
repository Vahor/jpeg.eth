use std::iter::Map;
use std::path::PathBuf;
use std::{env, fs};
use rayon::prelude::*;

use image::{imageops, DynamicImage, ImageBuffer};
use itertools::Itertools;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    version: u16,
    order: Vec<String>,
    input_size: u32,
    output_size: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct ArtLayerMeta {
    /// Offset from the top left corner of the image
    x_offset: i64,
    /// Offset from the top left corner of the image
    y_offset: i64,
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
        let a = ArtLayerMeta::from_json(&meta_data);

        println!("Meta data: {:?}", a);
        return a;
    }
}

struct ArtLayerElement {
    /// Name of the element (file name)
    name: String,
    /// The image of the element
    image: DynamicImage,
}

struct ArtLayer {
    /// Name of the layer (folder name)
    name: String,
    /// The meta data of the collection
    meta: ArtLayerMeta,
    /// The elements of the collection
    elements: Vec<ArtLayerElement>,
}

struct OutputImage {
    image: DynamicImage,
    attributes: Map<String, String>,
}

impl ArtLayer {
    fn from_folder(folder: PathBuf) -> ArtLayer {
        let folder_name = folder
            .file_name()
            .expect("Error reading folder name")
            .to_str()
            .expect("Error reading folder name");

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
                println!("Found image: {:?}", png_file.path());
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
                })
                .collect(),
        };

        layer
    }
}

fn combine_images(images: &Vec<(&ArtLayerElement, &ArtLayerMeta)>, config: &Config) -> DynamicImage {
    // Create a new image buffer
    // Iterate over each image
    // Copy the image into the new image buffer
    // Return the new image buffer

    let mut combined_image = ImageBuffer::new(config.input_size, config.input_size);

    for (element, meta) in images {
        let image = &element.image;
        let x_offset = meta.x_offset;
        let y_offset = meta.y_offset;

        imageops::overlay(&mut combined_image, image, x_offset, y_offset);
    }

    let combined_image = DynamicImage::ImageRgba8(combined_image);
    combined_image.resize(config.output_size, config.output_size, imageops::FilterType::Nearest);
    combined_image
}

fn create_combination(collections: Vec<&ArtLayer>) -> Vec<Vec<(&ArtLayerElement, &ArtLayerMeta)>> {
    let combination = collections
        .iter()
        .map(|collection| collection.elements.iter().zip(std::iter::repeat(&collection.meta)))
        .multi_cartesian_product()
        .collect::<Vec<_>>();

    combination
}

fn main() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let manifest_dir = PathBuf::from(manifest_dir);
    let input_folder = &manifest_dir.join("resources/input");

    let config_path = &manifest_dir.join("resources/config.json");
    let config = serde_json::from_str::<Config>(
        &fs::read_to_string(config_path).expect("Error reading config file"),
    ).expect("Error parsing config file");

    // config

    let folders = fs::read_dir(input_folder).expect("Error reading input folder");

    let output_folder = &manifest_dir.join("resources/output");
    fs::create_dir_all(output_folder).expect("Error creating output folder");

    let mut layers = Vec::new();

    // Iterate over each folder
    for folder in folders {
        if let Ok(folder_entry) = folder {
            // Skip if it's not a folder
            if !folder_entry
                .file_type()
                .expect("Error reading file type")
                .is_dir()
            {
                continue;
            }

            let layer = ArtLayer::from_folder(folder_entry.path());
            layers.push(layer);
        }
    }

    println!("Found {} layers", layers.len());

    // Create all combinations
    let combinations = create_combination(layers.iter().collect());

    // Combine all images
    combinations.par_iter().enumerate().for_each(|(index, combination)| {
        let image = combine_images(combination, &config);
        let file_name = format!("{}.png", index);
        let file_path = output_folder.join(file_name);
        image.save(file_path).expect("Error saving image");
        println!("Saved image {}", index);
    });
}
