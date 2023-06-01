use std::path::PathBuf;
use std::{env, fs};

use rayon::prelude::*;
use serde::{Deserialize, Serialize};

use crate::generator::{combine_images, create_combination};
use crate::layer::ArtLayer;

mod generator;
mod layer;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    version: u16,
    input_size: u32,
    output_size: u32,
}

fn main() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let manifest_dir = PathBuf::from(manifest_dir);
    let input_folder = &manifest_dir.join("resources/input");

    let config_path = &manifest_dir.join("resources/config.json");
    let config = serde_json::from_str::<Config>(
        &fs::read_to_string(config_path).expect("Error reading config file"),
    )
    .expect("Error parsing config file");

    let folders = fs::read_dir(input_folder).expect("Error reading input folder");

    let output_folder = &manifest_dir.join("resources/output");
    fs::create_dir_all(output_folder).expect("Error creating output folder");

    let mut layers = Vec::new();

    // Read all layers
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
    combinations.par_iter().for_each(|combination| {
        let image = combine_images(combination, &config);
        let image_path = output_folder.join(format!("{}.png", image.hash));
        let json_path = output_folder.join(format!("{}.json", image.hash));

        let attribute_str =
            serde_json::to_string(&image.attributes).expect("Error serializing attributes");

        image.image.save(image_path).expect("Error saving image");
        fs::write(json_path, attribute_str).expect("Error saving image attributes");

        println!("Saved image {}", image.hash);
    });

    println!("Saved {} images", combinations.len());
}
