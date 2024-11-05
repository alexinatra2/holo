mod holo;
mod interpolation;
mod parsing;

use chrono::Local;
use holo::{apply_holomorphic_function, SUPER_SAMPLING_FACTOR};
use image::{imageops::resize, RgbImage};
use parsing::{parse_holomorphic_function, parse_polynomial_expression};
use std::env;
use std::path::Path;

fn save_transformed_image(image_path: &str, coefficients: &[f64], transformed_img: RgbImage) {
    // Extract the file name (without the directory) from the input path
    let input_filename = Path::new(image_path)
        .file_stem()
        .expect("Failed to extract file name")
        .to_str()
        .expect("Failed to convert to string");

    // Create a string with coefficients separated by underscores
    let coefficients_str = coefficients
        .iter()
        .map(|coef| format!("{}", coef.round())) // Convert each coefficient to string
        .collect::<Vec<String>>()
        .join("_");

    let timestamp = Local::now().format("%Y%m%d%H%M%S").to_string();

    // Generate the output filename
    let output_filename = format!(
        "./images/output/{}_{}_{}.jpeg",
        input_filename, coefficients_str, timestamp
    );

    // Save the resulting image
    transformed_img
        .save(&output_filename)
        .expect("Failed to save image");

    println!("Image saved as: {}", output_filename);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!(
            "Usage: {} <image_name.extension> <coefficients...>",
            args[0]
        );
        return;
    }

    let image_file_path = Path::new(args[1].as_str());
    let image_file_name = image_file_path.file_name().unwrap();
    let image_path_string = format!("./images/output/{}", image_file_name.to_string_lossy());
    let image_path: &str = &image_path_string;

    // Check if the input contains a rational function (indicated by a '/')
    let input = args[2].as_str();
    let holomorphic_fn =
        parse_holomorphic_function(input).expect("Failed to parse polynomial coefficients");

    // Load the image
    let img = image::open(image_file_path)
        .expect("Failed to load image")
        .to_rgb8();

    // Super-sampling (Anti-Aliasing)
    // Step 1: Upscale the image
    let width = img.width() * SUPER_SAMPLING_FACTOR;
    let height = img.height() * SUPER_SAMPLING_FACTOR;
    let upscaled_img = resize(&img, width, height, image::imageops::FilterType::Lanczos3);

    // Step 2: Apply the holomorphic function to the upscaled image
    let transformed_img = apply_holomorphic_function(&upscaled_img, holomorphic_fn);

    // Step 3: Downscale the image back to the original size
    let final_img = resize(
        &transformed_img,
        img.width(),
        img.height(),
        image::imageops::FilterType::Lanczos3,
    );

    // Save the resulting image using the coefficients (for naming)
    let coefficients: Vec<f64> = if input.contains('/') {
        parse_polynomial_expression(input).expect("Failed to parse coefficients")
    } else {
        args[2..]
            .iter()
            .map(|s| s.parse::<f64>().expect("Failed to parse coefficient"))
            .collect()
    };

    save_transformed_image(image_path, &coefficients, final_img);
}
