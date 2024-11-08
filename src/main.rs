mod display;
mod holo;
mod parsing;
mod webcam;

use chrono::Local;
use clap::{arg, command, Parser};
use holo::HolomorphicLookup;
use image::{GenericImageView, RgbImage};
use parsing::parse_expression;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The filename to process (supports file completion in some shells)
    #[arg(value_name = "IMAGE_FILENAME", help = "Path to the file to process")]
    image: String,

    /// The function string to apply
    #[arg(
        value_name = "FUNCTION",
        help = "Function to apply to the file contents"
    )]
    function: String,
}

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
    let args = Cli::parse();

    let image_file_path = Path::new(&args.image);
    let image_file_name = image_file_path.file_name().unwrap();
    let image_path_string = format!("./images/output/{}", image_file_name.to_string_lossy());
    let image_path: &str = &image_path_string;

    // Join remaining arguments as a single string to support expressions with spaces
    let input = &args.function;
    let (_, holomorphic_fn) = parse_expression(input).expect("Failed to parse function expression");

    // Load the image
    let img = image::open(image_file_path)
        .expect("Failed to load image")
        .to_rgb8();

    let (width, height) = img.dimensions();

    let lookup = HolomorphicLookup::new(holomorphic_fn, width, height);
    if let Some(transformed_img) = lookup.apply(&img) {
        save_transformed_image(image_path, input, transformed_img);
    } else {
        eprint!("transforming image unsuccessful");
    }
}
