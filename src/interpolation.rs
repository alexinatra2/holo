use image::{Rgb, RgbImage};

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
