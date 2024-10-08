use image::Rgb;
use image::RgbImage;
use num_complex::Complex;
use wasm_bindgen::prelude::*;

// You can also export this for WebAssembly
#[wasm_bindgen]
pub fn transform_image_wasm(
    img_data: Vec<u8>, // image data as Vec<u8> for use in JS
    width: u32,
    height: u32,
    coefficients: Vec<f64>,
) -> Vec<u8> {
    let img = RgbImage::from_raw(width, height, img_data).expect("Invalid image data");
    let holomorphic_fn = construct_polynomial(coefficients);
    let transformed_img = apply_holomorphic_function(&img, holomorphic_fn);
    transformed_img.into_raw()
}

pub fn apply_holomorphic_function(
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
            // Normalize pixel positions to complex numbers
            let cx = (x as f64 - center_x) / center_x;
            let cy = (y as f64 - center_y) / center_y;
            let complex_pos = Complex::new(cx, cy);

            // Apply the holomorphic function to the complex position
            let result = f(complex_pos);

            // Map the result back to original image coordinates (inverse mapping)
            let orig_x = ((result.re * center_x) + center_x).round();
            let orig_y = ((result.im * center_y) + center_y).round();

            // If the original coordinates are within bounds, apply bilinear interpolation
            if orig_x >= 0.0
                && orig_y >= 0.0
                && orig_x < width as f64 - 1.0
                && orig_y < height as f64 - 1.0
            {
                // Perform bilinear interpolation
                let interpolated_pixel = bilinear_interpolation(img, orig_x, orig_y);
                transformed_img.put_pixel(x, y, interpolated_pixel);
            }
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
pub fn construct_polynomial(coefficients: Vec<f64>) -> impl Fn(Complex<f64>) -> Complex<f64> {
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
