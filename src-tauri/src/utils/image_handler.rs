use crate::models::contact::image::Image;
use crate::models::contact::image_details::ImageDetails;
use crate::models::contact::image_variants::ImageVariants;
use image::{DynamicImage, GenericImageView};
use std::fs;
use std::path::Path;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use sanitize_filename::sanitize;

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
        &[
            ImageFormat::Thumbnail,
            ImageFormat::Small,
            ImageFormat::Medium,
            ImageFormat::Large,
        ]
    }
}

const OUTPUT_PATH: &str = "./static/images";

pub fn generate_image_formats(icon: &mut ImageDetails) -> Result<(), String> {
    let img = load_image_from_path_or_base64(&icon.url)?;
    let mut formats = ImageVariants {
        thumbnail: None,
        small: None,
        medium: None,
        large: None,
    };

    // Remove the file extension before sanitizing the name
    let name_without_extension = Path::new(&icon.name)
        .file_stem()
        .unwrap_or_else(|| std::ffi::OsStr::new(&icon.name))
        .to_string_lossy();

    // Sanitize the resulting name
    let safe_name = sanitize(name_without_extension);
    let image_output_folder = format!("{}/{}", OUTPUT_PATH, safe_name);


    // Ensure the output path and image-specific folder exist
    fs::create_dir_all(&image_output_folder)
        .map_err(|err| format!("Failed to create output directory: {}", err))?;

    // Generate variants
    for format in ImageFormat::all_variants() {
        let (format_name, size) = format.data();

        // Resize the image
        let resized_img = img.resize(
            size as u32,
            size as u32,
            image::imageops::FilterType::Lanczos3,
        );

        // Create the output file path inside the sanitized folder
        let output_path = format!("{}/{}_{}.webp", image_output_folder, safe_name, format_name);

        // Save the resized image
        if let Err(err) = resized_img.save(&output_path) {
            return Err(format!("Failed to save {}: {}", format_name, err));
        }

        // Get image dimensions and size
        let (width, height) = resized_img.dimensions();
        let size_in_bytes = fs::metadata(&output_path)
            .map_err(|err| format!("Failed to get metadata for {}: {}", format_name, err))?
            .len() as f64;

        let variant_image = Image {
            name: format!("{}_{}.webp", safe_name, format_name),
            width: width as usize,
            height: height as usize,
            size: size_in_bytes,
            url: output_path.clone(),
        };

        // Assign to the appropriate variant
        match format_name {
            "thumbnail" => formats.thumbnail = Some(variant_image),
            "small" => formats.small = Some(variant_image),
            "medium" => formats.medium = Some(variant_image),
            "large" => formats.large = Some(variant_image),
            _ => {}
        }
    }

    // Update the icon formats
    icon.formats = Some(formats);

    Ok(())
}

fn load_image_from_path_or_base64(url: &str) -> Result<DynamicImage, String> {
    if url.starts_with("data:image/") && url.contains(";base64,") {
        // Extract the base64 string after the comma
        if let Some(base64_data) = url.split(',').nth(1) {
            // Decode the base64 data
            match STANDARD.decode(base64_data) {
                Ok(decoded_data) => {
                    // Load the image from the decoded byte array
                    image::load_from_memory(&decoded_data)
                        .map_err(|e| format!("Failed to load image from memory: {}", e))
                }
                Err(err) => Err(format!("Failed to decode base64: {}", err)),
            }
        } else {
            Err("Invalid base64 data URI.".to_string())
        }
    } else {
        // If not base64, assume it's a file path and try to open it
        image::open(url).map_err(|e| format!("Failed to open image file: {}", e))
    }
}
