use image::{Rgb, RgbImage};
use num_complex::Complex;

use crate::interpolation::bilinear_interpolation;

// Factor for super-sampling
pub const SUPER_SAMPLING_FACTOR: u32 = 4;
pub const SINGULARITY_THRESHOLD: f64 = 1e6; // Threshold to detect infinite values (singularity)
pub const FALLBACK_PIXEL: Rgb<u8> = Rgb([0, 0, 0]); // Fallback pixel (black)

// Apply holomorphic function with singularity handling
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
