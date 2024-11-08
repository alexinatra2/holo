use image::{Rgb, RgbImage};
use num_complex::Complex;

pub const SINGULARITY_THRESHOLD: f64 = 1e6; // Threshold to detect infinite values (singularity)
pub const FALLBACK_PIXEL: Rgb<u8> = Rgb([0, 0, 0]); // Fallback pixel (black)

/// Struct to hold the lookup table for holomorphic transformations
pub struct HolomorphicLookup {
    lookup: Vec<u32>,
    width: u32,
    height: u32,
    source_image: RgbImage, // Stores the image for transformations
}

impl HolomorphicLookup {
    /// Creates a new holomorphic lookup table for an image and a transformation function
    pub fn new(img: &RgbImage, f: impl Fn(Complex<f64>) -> Complex<f64>) -> Self {
        let (width, height) = img.dimensions();
        let center_x = width as f64 / 2.0;
        let center_y = height as f64 / 2.0;

        // Initialize lookup table vector
        let mut lookup = Vec::with_capacity((width * height) as usize);

        for y in 0..height {
            for x in 0..width {
                // Map pixel position to complex coordinates
                let cx = (x as f64 - center_x) / center_x;
                let cy = (y as f64 - center_y) / center_y;
                let complex_pos = Complex::new(cx, cy);

                // Apply holomorphic function
                let result = f(complex_pos);

                // Check for singularities
                if result.re.abs() > SINGULARITY_THRESHOLD
                    || result.im.abs() > SINGULARITY_THRESHOLD
                {
                    lookup.push((height - 1) * width); // Example index to indicate fallback
                    continue;
                }

                // Map the result back to original image coordinates
                let orig_x = result.re * center_x + center_x;
                let orig_y = result.im * center_y + center_y;

                // Handle quadrant wrapping
                let orig_x_extended = (orig_x + width as f64) % (width as f64 * 2.0);
                let orig_y_extended = (orig_y + height as f64) % (height as f64 * 2.0);

                let (final_x, final_y) =
                    if orig_x_extended < width as f64 && orig_y_extended < height as f64 {
                        (orig_x_extended, orig_y_extended)
                    } else if orig_x_extended >= width as f64 && orig_y_extended < height as f64 {
                        (
                            width as f64 - (orig_x_extended % width as f64),
                            orig_y_extended,
                        )
                    } else if orig_x_extended < width as f64 && orig_y_extended >= height as f64 {
                        (
                            orig_x_extended,
                            height as f64 - (orig_y_extended % height as f64),
                        )
                    } else {
                        (
                            width as f64 - (orig_x_extended % width as f64),
                            height as f64 - (orig_y_extended % height as f64),
                        )
                    };

                // Clamp coordinates and convert to index
                let final_x = final_x.clamp(width as f64 - 1.0, 0.0);
                let final_y = final_y.clamp(height as f64 - 1.0, 0.0);

                // Calculate and store the index for the transformed coordinates
                let pixel_index = final_y * width + final_x;
                lookup.push(pixel_index);
            }
        }

        HolomorphicLookup {
            lookup,
            width,
            height,
            source_image: img.clone(),
        }
    }

    /// Sets the source image to be used for transformations
    pub fn set_image(&mut self, img: RgbImage) {
        self.source_image = img;
    }

    /// Retrieve the mapped pixel index for a given (x, y) position
    pub fn get(&self, x: u32, y: u32) -> Option<u32> {
        if x < self.width && y < self.height {
            let index = (y * self.width + x) as usize;
            self.lookup.get(index).cloned()
        } else {
            None
        }
    }

    /// Applies the lookup to transform the image based on the precomputed positions
    pub fn apply(&self) -> Option<RgbImage> {
        // Check that a source image has been set
        let source_image = &self.source_image;

        let mut transformed_img = RgbImage::new(self.width, self.height);

        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(mapped_index) = self.get(x, y) {
                    let orig_x = mapped_index % self.width;
                    let orig_y = mapped_index / self.width;
                    let pixel = source_image.get_pixel(orig_x, orig_y);
                    transformed_img.put_pixel(x, y, *pixel);
                } else {
                    // Fallback if lookup fails (optional)
                    transformed_img.put_pixel(x, y, FALLBACK_PIXEL);
                }
            }
        }

        Some(transformed_img)
    }
}
