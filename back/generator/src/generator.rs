
use image::{imageops, DynamicImage, ImageBuffer};
use itertools::Itertools;
use sha2::{Digest, Sha256};
use serde::{Serialize};

use crate::layer::{ArtLayer, ArtLayerElement, ArtLayerMeta};
use crate::Config;

pub struct OutputImage {
    /// The hash of the attributes of the image
    pub hash: String,
    /// The image itself
    pub image: DynamicImage,
    /// The attributes of the image
    pub attributes: Vec<Attribute>,
}

#[derive(Serialize)]
pub struct Attribute {
    pub trait_type: String,
    pub value: String,
}

/// Take multiple layers and combine them into a single image.
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

    // We want to sort the attributes so that the hash is always the same
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

    let attributes = attributes.into_iter().map(|(k, v)| {
        Attribute {
            trait_type: k,
            value: v,
        }
    }).collect();

    OutputImage {
        hash: format!("{:x}", hash),
        image: combined_image,
        attributes,
    }
}

/// Return all possible combinations for the given collections
///
/// Ex: [['red background', 'blue background'], ['red circle', 'blue circle']] => [['red background', 'red circle'], ['red background', 'blue circle'], ['blue background', 'red circle'], ['blue background', 'blue circle']]
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
