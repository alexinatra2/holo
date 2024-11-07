mod holo;
mod parsing;

use chrono::Local;
use holo::apply_holomorphic_function;
use image::RgbImage;
use parsing::parse_expression;
use std::env;
use std::path::Path;

fn save_transformed_image(image_path: &str, function_str: &str, transformed_img: RgbImage) {
    // Extract the file name (without directory) from the input path
    let input_filename = Path::new(image_path)
        .file_stem()
        .expect("Failed to extract file name")
        .to_str()
        .expect("Failed to convert to string");

    let timestamp = Local::now().format("%Y%m%d%H%M%S").to_string();

    // Replace '/' with 'div' for safe filename construction
    let sanitized_function_str: String = function_str
        .replace("/", "div")
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();

    // Generate the output filename
    let output_filename = format!(
        "./images/output/{}_{}_{}.jpeg",
        input_filename, sanitized_function_str, timestamp
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

    // Join remaining arguments as a single string to support expressions with spaces
    let input = args[2].as_str();
    let (_, holomorphic_fn) = parse_expression(input).expect("Failed to parse function expression");

    // Load the image
    let img = image::open(image_file_path)
        .expect("Failed to load image")
        .to_rgb8();

    let transformed_img = apply_holomorphic_function(&img, holomorphic_fn);

    save_transformed_image(image_path, input, transformed_img);
}
