use image::{imageops::resize, Rgb, RgbImage};
use num_complex::Complex;

// Factor for super-sampling
pub const SUPER_SAMPLING_FACTOR: u32 = 2;
pub const SINGULARITY_THRESHOLD: f64 = 1e6; // Threshold to detect infinite values (singularity)
pub const FALLBACK_PIXEL: Rgb<u8> = Rgb([0, 0, 0]); // Fallback pixel (black)

// Apply holomorphic function with singularity handling
pub fn apply_holomorphic_function(
    img: &RgbImage,
    f: impl Fn(Complex<f64>) -> Complex<f64>,
) -> RgbImage {
    let width = img.width() * SUPER_SAMPLING_FACTOR;
    let height = img.height() * SUPER_SAMPLING_FACTOR;
    let upscaled_img = resize(img, width, height, image::imageops::FilterType::Lanczos3);
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

            // Apply the holomorphic function to find the corresponding pixel position
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

            // Wrap coordinates to extend into three additional quadrants
            let orig_x_extended = (orig_x + width as f64) % (width as f64 * 2.0);
            let orig_y_extended = (orig_y + height as f64) % (height as f64 * 2.0);

            // Determine the final coordinates based on the quadrant
            let (final_x, final_y) =
                if orig_x_extended < width as f64 && orig_y_extended < height as f64 {
                    // Quadrant I (original)
                    (orig_x_extended, orig_y_extended)
                } else if orig_x_extended >= width as f64 && orig_y_extended < height as f64 {
                    // Quadrant II (flipped across y-axis)
                    (
                        width as f64 - (orig_x_extended % width as f64),
                        orig_y_extended,
                    )
                } else if orig_x_extended < width as f64 && orig_y_extended >= height as f64 {
                    // Quadrant III (flipped across x-axis)
                    (
                        orig_x_extended,
                        height as f64 - (orig_y_extended % height as f64),
                    )
                } else {
                    // Quadrant IV (flipped across both axes)
                    (
                        width as f64 - (orig_x_extended % width as f64),
                        height as f64 - (orig_y_extended % height as f64),
                    )
                };

            // Clamp the final coordinates to ensure they are within bounds
            let final_x = final_x.max(0.0).min(width as f64 - 1.0) as u32;
            let final_y = final_y.max(0.0).min(height as f64 - 1.0) as u32;

            // Sample the image at the computed source pixel
            let sampled_pixel =
                bilinear_interpolation(&upscaled_img, final_x as f64, final_y as f64);

            // Set the transformed pixel
            transformed_img.put_pixel(x, y, sampled_pixel);
        }
    }
    // Step 3: Downscale the image back to the original size
    let final_img = resize(
        &transformed_img,
        img.width(),
        img.height(),
        image::imageops::FilterType::Lanczos3,
    );

    final_img
}

// Bilinear interpolation function
pub fn bilinear_interpolation(img: &RgbImage, x: f64, y: f64) -> Rgb<u8> {
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
