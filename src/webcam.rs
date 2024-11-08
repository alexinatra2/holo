use image::{Rgb, RgbImage};
use opencv::core::Mat;
use opencv::prelude::*;
use opencv::videoio::VideoCapture;

pub fn capture_frame(cap: &mut VideoCapture) -> Option<Mat> {
    let mut frame = Mat::default();
    if cap.read(&mut frame).unwrap() && !frame.empty() {
        // Optionally resize or preprocess frame if needed
        Some(frame)
    } else {
        None
    }
}

pub fn mat_to_rgb_image(mat: &Mat) -> Option<RgbImage> {
    let mut rgb_image = RgbImage::new(mat.cols() as u32, mat.rows() as u32);
    for y in 0..mat.rows() {
        for x in 0..mat.cols() {
            let pixel = mat.at_2d::<opencv::core::Vec3b>(y, x).ok()?;
            rgb_image.put_pixel(
                x as u32,
                y as u32,
                Rgb([pixel[2], pixel[1], pixel[0]]), // Convert BGR to RGB
            );
        }
    }
    Some(rgb_image)
}
