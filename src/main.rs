mod cli;
mod display;
mod holo;
mod parsing;
mod webcam;

use chrono::Local;
use clap::Parser as ClapParser;
use cli::Cli;
use display::display_image;
use holo::{process_frame, HolomorphicLookup};
use image::RgbImage;
use minifb::{Key, Window, WindowOptions};
use opencv::videoio::{
    VideoCapture, VideoCaptureTrait, CAP_ANY, CAP_PROP_FRAME_HEIGHT, CAP_PROP_FRAME_WIDTH,
};
use parsing::parse_and_generate_closure;
use std::path::Path;
use webcam::capture_frame;

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    let input = &args.function;
    let holomorphic_fn = parse_and_generate_closure(input)?;

    match args.image {
        Some(file_path) => {
            let image_file_path = Path::new(&file_path);
            let image_file_name = image_file_path.file_name().unwrap();
            let image_path_string =
                format!("./images/output/{}", image_file_name.to_string_lossy());
            let image_path: &str = &image_path_string;

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
        None => {
            // Determine width and height based on resolution or dimensions
            let (width, height) = if let Some(res) = args.resolution {
                res.to_dimensions()
            } else {
                args.dimensions.unwrap_or((640, 480)) // Default dimensions if none provided
            };
            let lookup = HolomorphicLookup::new(holomorphic_fn, width, height);
            let mut cap = VideoCapture::new(0, CAP_ANY)?; // 0 is the default camera
            cap.set(CAP_PROP_FRAME_WIDTH, width as f64)?;
            cap.set(CAP_PROP_FRAME_HEIGHT, height as f64)?;

            // Create display window
            let mut window = Window::new(
                "Holomorphic Webcam",
                width as usize,
                height as usize,
                WindowOptions::default(),
            )
            .expect("Failed to create window");
            while window.is_open() && !window.is_key_down(Key::Escape) {
                // Capture frame from webcam
                if let Some(frame) = capture_frame(&mut cap) {
                    // Apply transformation
                    if let Some(transformed_image) = process_frame(&lookup, &frame) {
                        // Display the transformed frame
                        display_image(&mut window, &transformed_image);
                    }
                }
            }
        }
    }

    Ok(())
}
