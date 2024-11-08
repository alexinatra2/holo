use image::RgbImage;
use minifb::Window;

pub fn display_image(window: &mut Window, img: &RgbImage) {
    let buffer: Vec<u32> = img
        .pixels()
        .map(|p| {
            let (r, g, b) = (p[0] as u32, p[1] as u32, p[2] as u32);
            (r << 16) | (g << 8) | b
        })
        .collect();

    window
        .update_with_buffer(&buffer, img.width() as usize, img.height() as usize)
        .unwrap();
}
