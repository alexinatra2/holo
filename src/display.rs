use image::RgbImage;
use minifb::Window;

/// Zeigt ein Bild in einem Minifb-Fenster an.
///
/// Diese Funktion nimmt ein Bild im `RgbImage`-Format und wandelt es in ein Pufferformat (`Vec<u32`) um,
/// das mit der `minifb`-Bibliothek kompatibel ist. Das Bild wird dann in einem Fenster angezeigt.
///
/// # Parameter
/// - `window` (`&mut Window`): Das Fenster, in dem das Bild angezeigt wird.
/// - `img` (`&RgbImage`): Das Bild im RGB-Format, das angezeigt werden soll.
///
/// # Funktionsweise
/// 1. Die Pixel des Bildes werden gelesen und jedes Pixel in ein `u32`-Format umgewandelt:
///    - Rot (`R`) wird in die höchsten 8 Bits geschoben (`<< 16`).
///    - Grün (`G`) wird in die mittleren 8 Bits geschoben (`<< 8`).
///    - Blau (`B`) bleibt in den niedrigsten 8 Bits.
/// 2. Der resultierende Vektor (`Vec<u32>`) wird verwendet, um den Puffer des Fensters zu aktualisieren.
///
/// # Fehler
/// Diese Funktion kann paniken, wenn:
/// - Das Fenster-Update mit dem Puffer fehlschlägt (`unwrap()` wird verwendet).
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
