mod holo;
mod interpolation;
mod parsing;
mod polynomial;

use holo::{apply_holomorphic_function, SUPER_SAMPLING_FACTOR};
use image::{imageops::resize, RgbImage};
use num_complex::Complex;
use parsing::parse_polynomial_expression;
use polynomial::{construct_polynomial, construct_rational_function};
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
        .map(|coef| format!("{:.1}", coef)) // Convert each coefficient to string
        .collect::<Vec<String>>()
        .join("_");

    // Generate the output filename
    let output_filename = format!(
        "./images/output/{}_{}.jpeg",
        input_filename, coefficients_str
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

    let image_path_string = format!("./images/input/{}", args[1].as_str());
    let image_path: &str = &image_path_string;

    // Check if the input contains a rational function (indicated by a '/')
    let input = args[2].as_str();
    let holomorphic_fn: Box<dyn Fn(Complex<f64>) -> Complex<f64>>;

    if input.contains('/') {
        // Parse as a rational function
        let parts: Vec<&str> = input.split('/').collect();
        if parts.len() != 2 {
            eprintln!("Invalid input format for rational function");
            return;
        }

        // Parse numerator and denominator coefficients
        let numerator = parse_polynomial_expression(parts[0]).expect("Failed to parse numerator");
        let denominator =
            parse_polynomial_expression(parts[1]).expect("Failed to parse denominator");

        holomorphic_fn = Box::new(construct_rational_function(numerator, denominator));
    } else {
        // Parse as a polynomial if no '/' is found (coefficients provided as arguments)
        if args.len() < 3 {
            eprintln!("Please provide coefficients for the polynomial.");
            return;
        }

        // Collect coefficients from args[2..] and convert them to f64
        let coefficients: Vec<f64> = args[2..]
            .iter()
            .map(|s| s.parse::<f64>().expect("Failed to parse coefficient"))
            .collect();

        holomorphic_fn = Box::new(construct_polynomial(coefficients.clone())); // Use clone here to pass to save function
    }

    // Load the image
    let img = image::open(image_path)
        .expect("Failed to load image")
        .to_rgb8();

    //SUPER-SAMPLING (ANTI-ALIASING)
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
    // For rational functions
    } else {
        args[2..]
            .iter()
            .map(|s| s.parse::<f64>().expect("Failed to parse coefficient"))
            .collect() // For normal polynomials
    };

    save_transformed_image(image_path, &coefficients, final_img);
}
