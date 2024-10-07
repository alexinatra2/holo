use image::RgbImage;
use num_complex::Complex;

fn apply_holomorphic_function(img: &RgbImage, f: fn(Complex<f64>) -> Complex<f64>) -> RgbImage {
    // Create a new image to store the transformed result
    let mut transformed_img = RgbImage::new(img.width(), img.height());

    // Get image dimensions
    let (width, height) = img.dimensions();

    // Compute the center of the image to normalize coordinates
    let center_x = width as f64 / 2.0;
    let center_y = height as f64 / 2.0;

    // Iterate over each pixel position (x, y)
    for (x, y, pixel) in img.enumerate_pixels() {
        // Normalize pixel positions to complex numbers relative to the image center
        let cx = (x as f64 - center_x) / center_x;
        let cy = (y as f64 - center_y) / center_y;
        let complex_pos = Complex::new(cx, cy);

        // Apply the holomorphic function to the complex position
        let result = f(complex_pos);

        // Map the result back to image coordinates
        let new_x = ((result.re * center_x) + center_x).round() as u32;
        let new_y = ((result.im * center_y) + center_y).round() as u32;

        // Ensure the new coordinates are within bounds
        if new_x < width && new_y < height {
            // Set the pixel at the new position to the original pixel color
            transformed_img.put_pixel(new_x, new_y, *pixel);
        }
    }

    transformed_img
}

fn main() {
    // Load the image
    let img_data: &[u8] = include_bytes!("./images/input/test_image.jpg");
    let img = image::load_from_memory(img_data)
        .expect("Failed to load image from bytes")
        .to_rgb8();

    // Define a simple holomorphic function (f(z) = z^2)
    let holomorphic_fn = |z: Complex<f64>| z * z;

    // Apply the holomorphic function to the image
    let transformed_img = apply_holomorphic_function(&img, holomorphic_fn);

    // Save the resulting image
    transformed_img
        .save("./output/test_image.jpeg")
        .expect("Failed to save image");
}
