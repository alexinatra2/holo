use image::{Rgb, RgbImage};
use opencv::core::Mat;
use opencv::prelude::*;
use opencv::videoio::VideoCapture;

/// Erfasst ein einzelnes Frame von der Webcam oder einer Videoquelle.
///
/// Diese Funktion liest ein Bild von der angegebenen Videoquelle (`VideoCapture`)
/// und gibt es als OpenCV-Mat-Objekt zurück, wenn das Frame erfolgreich erfasst wurde.
///
/// # Parameter
/// - `cap` (`&mut VideoCapture`): Die Videoquelle, z. B. eine Webcam oder ein Videostream.
///
/// # Rückgabewert
/// Gibt ein `Mat`-Objekt zurück, das das erfasste Bild enthält, oder `None`, wenn kein Frame erfasst werden konnte.
///
/// # Fehler
/// - Gibt `None` zurück, wenn das Frame leer ist oder ein Fehler beim Lesen auftritt.
pub fn capture_frame(cap: &mut VideoCapture) -> Option<Mat> {
    let mut frame = Mat::default();
    if cap.read(&mut frame).unwrap() && !frame.empty() {
        // Optional: Frame skalieren oder vorverarbeiten
        Some(frame)
    } else {
        None
    }
}

/// Konvertiert ein OpenCV-Mat-Bild in ein `RgbImage`.
///
/// Diese Funktion nimmt ein OpenCV-Bild (im BGR-Format) und konvertiert es
/// in ein `RgbImage` aus der `image`-Crate, indem die Farbkanäle von BGR zu RGB umgeordnet werden.
///
/// # Parameter
/// - `mat` (`&Mat`): Das OpenCV-Mat-Objekt, das konvertiert werden soll.
///
/// # Rückgabewert
/// Gibt ein `RgbImage` zurück, das das konvertierte Bild enthält, oder `None`, wenn ein Fehler auftritt.
///
/// # Fehler
/// - Gibt `None` zurück, wenn Pixel nicht aus dem Mat-Objekt extrahiert werden können.
pub fn mat_to_rgb_image(mat: &Mat) -> Option<RgbImage> {
    let mut rgb_image = RgbImage::new(mat.cols() as u32, mat.rows() as u32);
    for y in 0..mat.rows() {
        for x in 0..mat.cols() {
            let pixel = mat.at_2d::<opencv::core::Vec3b>(y, x).ok()?;
            rgb_image.put_pixel(
                x as u32,
                y as u32,
                Rgb([pixel[2], pixel[1], pixel[0]]), // Konvertiere BGR zu RGB
            );
        }
    }
    Some(rgb_image)
}
