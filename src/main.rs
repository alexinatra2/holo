use image::Rgb;
use image::RgbImage;
use num_complex::Complex;
use std::env;
use std::path::Path;

fn apply_holomorphic_function(
    img: &RgbImage,
    f: impl Fn(Complex<f64>) -> Complex<f64>,
) -> RgbImage {
    let (width, height) = img.dimensions();
    let mut transformed_img = RgbImage::new(width, height);

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

// Bilinear interpolation function
fn bilinear_interpolation(img: &RgbImage, x: f64, y: f64) -> Rgb<u8> {
    let x0 = x.floor() as u32;
    let x1 = x.ceil() as u32;
    let y0 = y.floor() as u32;
    let y1 = y.ceil() as u32;

    let px00 = img.get_pixel(x0, y0);
    let px01 = img.get_pixel(x0, y1);
    let px10 = img.get_pixel(x1, y0);
    let px11 = img.get_pixel(x1, y1);

    let x_weight = x - x0 as f64;
    let y_weight = y - y0 as f64;

    let interpolate =
        |a: u8, b: u8, weight: f64| -> u8 { ((1.0 - weight) * a as f64 + weight * b as f64) as u8 };

    let interpolate_rgb = |c1: &Rgb<u8>, c2: &Rgb<u8>, weight: f64| -> Rgb<u8> {
        Rgb([
            interpolate(c1[0], c2[0], weight),
            interpolate(c1[1], c2[1], weight),
            interpolate(c1[2], c2[2], weight),
        ])
    };

    // Interpolate the four surrounding pixels
    let top = interpolate_rgb(px00, px10, x_weight);
    let bottom = interpolate_rgb(px01, px11, x_weight);
    interpolate_rgb(&top, &bottom, y_weight)
}

// Function to construct a holomorphic polynomial from the coefficients
fn construct_polynomial(coefficients: Vec<f64>) -> impl Fn(Complex<f64>) -> Complex<f64> {
    move |z: Complex<f64>| {
        let mut result = Complex::new(0.0, 0.0);
        let mut z_pow = Complex::new(1.0, 0.0); // This is z^0 initially
        for &coef in &coefficients {
            result += coef * z_pow;
            z_pow *= z; // Increment to the next power of z
        }
        result
    }
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

fn main() {
    // Get command-line arguments: first is image path, rest are polynomial coefficients
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <image_path> <coefficients...>", args[0]);
        return;
    }

    let image_path = &args[1];

    // Parse the remaining arguments as coefficients for the polynomial
    let coefficients: Vec<f64> = args[2..]
        .iter()
        .map(|arg| arg.parse::<f64>().expect("Invalid coefficient"))
        .collect();

    // Construct the polynomial function
    let holomorphic_fn = construct_polynomial(coefficients.clone());

    // Load the image
    let img = image::open(image_path)
        .expect("Failed to load image")
        .to_rgb8();

    // Apply the holomorphic function to the image
    let transformed_img = apply_holomorphic_function(&img, holomorphic_fn);

    // Save the resulting image using the new function
    save_transformed_image(image_path, &coefficients, transformed_img);
}
