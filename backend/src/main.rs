use backend_lib::{apply_holomorphic_function, construct_polynomial};
use image::RgbImage;
use std::env;
use std::path::Path; // Import from lib.rs
                     //
fn main() {
    // CLI logic to get image path and polynomial coefficients from arguments
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <image_path> <coefficients...>", args[0]);
        return;
    }

    let image_path = &args[1];
    let coefficients: Vec<f64> = args[2..]
        .iter()
        .map(|arg| arg.parse::<f64>().expect("Invalid coefficient"))
        .collect();

    // Load the image
    let img = image::open(image_path)
        .expect("Failed to load image")
        .to_rgb8();

    // Apply the holomorphic function
    let holomorphic_fn = construct_polynomial(coefficients.clone());
    let transformed_img = apply_holomorphic_function(&img, holomorphic_fn);

    // Save the transformed image
    save_transformed_image(image_path, &coefficients, transformed_img);
}

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
        .map(|coef| format!("{:.1}", coef)) // Convert each coefficient to string
        .collect::<Vec<String>>()
        .join("_");

    // Generate the output filename
    let output_filename = format!("./output/{}_{}.jpeg", input_filename, coefficients_str);

    // Save the resulting image
    transformed_img
        .save(&output_filename)
        .expect("Failed to save image");

    println!("Image saved as: {}", output_filename);
}
