use std::collections::HashMap;

use image::{imageops, DynamicImage, ImageBuffer};
use itertools::Itertools;
use sha2::{Digest, Sha256};

use crate::layer::{ArtLayer, ArtLayerElement, ArtLayerMeta};
use crate::Config;

pub struct OutputImage {
    pub hash: String,
    pub image: DynamicImage,
    pub attributes: HashMap<String, String>,
}

pub fn combine_images(
    images: &Vec<(&ArtLayerElement, &ArtLayerMeta)>,
    config: &Config,
) -> OutputImage {
    let mut combined_image = ImageBuffer::new(config.input_size, config.input_size);

    for (element, meta) in images {
        let image = &element.image;
        let x_offset = meta.x_offset;
        let y_offset = meta.y_offset;

        imageops::overlay(&mut combined_image, image, x_offset, y_offset);
    }

    let combined_image = DynamicImage::ImageRgba8(combined_image);
    combined_image.resize(
        config.output_size,
        config.output_size,
        imageops::FilterType::Nearest,
    );

    // sort the attributes by key, then join them into a string, then hash the string

    let attributes = images
        .into_iter()
        .map(|(element, _)| {
            (
                element.element_type.clone(),
                element
                    .name
                    .clone()
                    .splitn(2, ".")
                    .next()
                    .unwrap()
                    .to_string(),
            )
        })
        .sorted()
        .collect::<Vec<_>>();

    let mut hasher = Sha256::new();
    hasher.update(format!("{:?}", attributes).as_bytes());
    let hash = hasher.finalize();

    let attributes: HashMap<String, String> = HashMap::from_iter(attributes);

    OutputImage {
        hash: format!("{:x}", hash),
        image: combined_image,
        attributes,
    }
}

pub fn create_combination(
    collections: Vec<&ArtLayer>,
) -> Vec<Vec<(&ArtLayerElement, &ArtLayerMeta)>> {
    let combination = collections
        .iter()
        .map(|collection| {
            collection
                .elements
                .iter()
                .zip(std::iter::repeat(&collection.meta))
        })
        .multi_cartesian_product()
        .collect::<Vec<_>>();

    combination
}
