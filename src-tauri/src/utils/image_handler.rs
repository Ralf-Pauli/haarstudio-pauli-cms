use std::fs;
use crate::models::contact::image_details::ImageDetails;
use image::{DynamicImage, GenericImageView};
use crate::models::contact::image::Image;
use crate::models::contact::image_variants::ImageVariants;

enum ImageFormat {
    Thumbnail,
    Large,
    Medium,
    Small,
}

impl ImageFormat {
    fn data(&self) -> (&str, usize) {
        match self {
            ImageFormat::Thumbnail => ("thumbnail", 156),
            ImageFormat::Small => ("small", 500),
            ImageFormat::Medium => ("medium", 750),
            ImageFormat::Large => ("large", 1000),
        }
    }

    fn all_variants() -> &'static [ImageFormat] {
        &[ImageFormat::Thumbnail, ImageFormat::Small, ImageFormat::Medium, ImageFormat::Large]
    }
}


const OUTPUT_PATH: &str = "static/images/";

pub fn generate_image_formats(icon: &mut ImageDetails) -> Result<(), String> {
    let img = match image::open(&icon.url) {
        Ok(img) => img,
        Err(err) => return Err(format!("Failed to open image: {}", err))
    };

    let mut formats = ImageVariants {
        thumbnail: None,
        small: None,
        medium: None,
        large: None,
    };


    fs::create_dir_all(OUTPUT_PATH).map_err(|err| format!("Failed to create output directory: {}", err))?;

    for format in ImageFormat::all_variants() {
        let (format_name, size) = format.data();

        let resized_img = img.resize(size as u32, size as u32, image::imageops::FilterType::Lanczos3);
        let output_path = format!("{}/{}_{}.webp", OUTPUT_PATH, icon.name, format_name);

        if let Err(err) = resized_img.save(&output_path) {
            return Err(format!("Failed to save {}: {}", format_name, err));
        }

        let (width, height) = resized_img.dimensions();
        let size_in_bytes = fs::metadata(&output_path)
            .map_err(|err| format!("Failed to get metadata for {}: {}", format_name, err))?
            .len() as f64;

        let variant_image = Image {
            name: format!("{}_{}.webp", icon.name, format_name),
            width: width as usize,
            height: height as usize,
            size: size_in_bytes,
            url: output_path.clone()
        };

        match format_name {
            "thumbnail" => formats.thumbnail = Some(variant_image),
            "small" => formats.small = Some(variant_image),
            "medium" => formats.medium = Some(variant_image),
            "large" => formats.large = Some(variant_image),
            _ => {}
        }
    }

    icon.formats = Some(formats);

    Ok(())
}

fn transform_to_webp() {}

fn update_image_details() {}
