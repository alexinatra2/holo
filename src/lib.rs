mod holo;
mod parsing;

use std::io::Cursor;

use holo::apply_holomorphic_function;
use image::{codecs::png::PngEncoder, ExtendedColorType, ImageEncoder, RgbImage};
use parsing::parse_expression;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn transform_image(image_data: Vec<u8>, func_str: String, width: u32, height: u32) -> Vec<u8> {
    // Parse the holomorphic function from the string
    let parsed_function = match parse_expression(&func_str) {
        Ok((_, func)) => func,
        Err(e) => {
            // Return an empty vector or handle the error (invalid function)
            eprintln!("Error parsing function: {:?}", e);
            return Vec::new();
        }
    };

    // Convert the input image data into an image object
    let img = RgbImage::from_raw(width, height, image_data).unwrap();

    // Apply the holomorphic function to the image
    let transformed_img = apply_holomorphic_function(&img, parsed_function);

    // Convert the transformed image back to a byte vector for returning as a result
    let mut transformed_data = Vec::new();
    let mut cursor = Cursor::new(&mut transformed_data);
    // Create a PNG encoder and write the image data to the cursor
    let encoder = PngEncoder::new(&mut cursor);
    encoder
        .write_image(
            transformed_img.as_raw(),
            transformed_img.width(),
            transformed_img.height(),
            ExtendedColorType::Rgb8,
        )
        .expect("Failed to encode PNG image");

    transformed_data
}
