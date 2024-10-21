use image::{imageops::resize, Rgb, RgbImage};
use num_complex::Complex;
use std::env;
use std::path::Path;
use regex::Regex;
use std::str::FromStr;
use std::num::ParseFloatError;

// Factor for super-sampling
const SUPER_SAMPLING_FACTOR: u32 = 4;
const SINGULARITY_THRESHOLD: f64 = 1e6; // Threshold to detect infinite values (singularity)
const FALLBACK_PIXEL: Rgb<u8> = Rgb([0, 0, 0]); // Fallback pixel (black)

// Apply holomorphic function with singularity handling
fn apply_holomorphic_function(
    img: &RgbImage,
    f: impl Fn(Complex<f64>) -> Complex<f64>,
) -> RgbImage {
    let (width, height) = img.dimensions();
    let mut transformed_img = RgbImage::new(width, height);

    let center_x = width as f64 / 2.0;
    let center_y = height as f64 / 2.0;

    // Iterate over each pixel in the transformed image
    for y in 0..height {
        for x in 0..width {
            // Map the pixel in the target image back to a complex number
            let cx = (x as f64 - center_x) / center_x;
            let cy = (y as f64 - center_y) / center_y;
            let complex_pos = Complex::new(cx, cy);

            // Apply the inverse of the holomorphic function to find the corresponding source pixel
            let result = f(complex_pos);

            // Check for singularities (infinite values or extremely large values)
            if result.re.abs() > SINGULARITY_THRESHOLD || result.im.abs() > SINGULARITY_THRESHOLD {
                // Use fallback pixel if a singularity is detected
                transformed_img.put_pixel(x, y, FALLBACK_PIXEL);
                continue;
            }

            // Map the result back to original image coordinates => INVERSE MAPPING
            let orig_x = result.re * center_x + center_x;
            let orig_y = result.im * center_y + center_y;

            // Clamp the coordinates to ensure they are within bounds
            let orig_x_clamped = orig_x.max(0.0).min(width as f64 - 1.0);
            let orig_y_clamped = orig_y.max(0.0).min(height as f64 - 1.0);

            // Sample the image at the computed source pixel, with bilinear interpolation
            let sampled_pixel = bilinear_interpolation(img, orig_x_clamped, orig_y_clamped);

            // Set the transformed pixel
            transformed_img.put_pixel(x, y, sampled_pixel);
        }
    }

    transformed_img
}

// Bilinear interpolation function
fn bilinear_interpolation(img: &RgbImage, x: f64, y: f64) -> Rgb<u8> {
    let x0 = x.floor() as u32;
    let x1 = (x0 + 1).min(img.width() - 1);
    let y0 = y.floor() as u32;
    let y1 = (y0 + 1).min(img.height() - 1);

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

// Function to construct a rational function from numerator and denominator coefficients
fn construct_rational_function(
    numerator: Vec<f64>,
    denominator: Vec<f64>,
) -> impl Fn(Complex<f64>) -> Complex<f64> {
    move |z: Complex<f64>| {
        let mut num_result = Complex::new(0.0, 0.0);
        let mut denom_result = Complex::new(0.0, 0.0);

        // Calculate numerator
        let mut z_pow = Complex::new(1.0, 0.0); // Start with z^0
        for &coef in &numerator {
            num_result += coef * z_pow;
            z_pow *= z;
        }

        // Calculate denominator
        z_pow = Complex::new(1.0, 0.0); // Reset to z^0
        for &coef in &denominator {
            denom_result += coef * z_pow;
            z_pow *= z;
        }

        // Handle division by zero or very small denominator values
        if denom_result.norm() < 1e-6 {
            Complex::new(0.0, 0.0) // Return 0 if division by zero occurs
        } else {
            num_result / denom_result
        }
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
    let output_filename = format!("./images/output/{}_{}.jpeg", input_filename, coefficients_str);

    // Save the resulting image
    transformed_img
        .save(&output_filename)
        .expect("Failed to save image");

    println!("Image saved as: {}", output_filename);
}

// Funktion zum Parsen der Polynomausdrücke (z.B. "3z + 2z^2 + 3")
fn parse_polynomial_expression(input: &str) -> Result<Vec<f64>, ParseFloatError> {
    let term_regex = Regex::new(r"([+-]?\d*\.?\d*)z(?:\^(\d+))?").unwrap();
    let constant_regex = Regex::new(r"([+-]?\d*\.?\d+)$").unwrap();

    let mut coefficients = vec![];

    // Find all polynomial terms like "3z^2", "2z", etc.
    for cap in term_regex.captures_iter(input) {
        let coefficient_str = cap.get(1).map_or("1", |m| m.as_str()); // Default coefficient is 1 if omitted
        let exponent_str = cap.get(2).map_or("1", |m| m.as_str()); // Default exponent is 1 if omitted
        let coefficient: f64 = if coefficient_str == "+" || coefficient_str == "" {
            1.0
        } else if coefficient_str == "-" {
            -1.0
        } else {
            f64::from_str(coefficient_str)?
        };
        let exponent: usize = exponent_str.parse().unwrap_or(1);

        // Ensure the coefficients vector is large enough
        if coefficients.len() <= exponent {
            coefficients.resize(exponent + 1, 0.0);
        }

        // Add the coefficient to the corresponding power of z
        coefficients[exponent] = coefficient;
    }

    // Find any constant term (no z)
    if let Some(cap) = constant_regex.captures(input) {
        let constant: f64 = f64::from_str(cap.get(1).unwrap().as_str())?;
        if coefficients.is_empty() {
            coefficients.push(constant);
        } else {
            coefficients[0] = constant;
        }
    }

    Ok(coefficients)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <image_name.extension> <coefficients...>", args[0]);
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
        let denominator = parse_polynomial_expression(parts[1]).expect("Failed to parse denominator");

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
    let final_img = resize(&transformed_img, img.width(), img.height(), image::imageops::FilterType::Lanczos3);

    // Save the resulting image using the coefficients (for naming)
    let coefficients: Vec<f64> = if input.contains('/') {
        parse_polynomial_expression(input).expect("Failed to parse coefficients") // For rational functions
    } else {
        args[2..]
            .iter()
            .map(|s| s.parse::<f64>().expect("Failed to parse coefficient"))
            .collect() // For normal polynomials
    };

    save_transformed_image(image_path, &coefficients, final_img);
}
