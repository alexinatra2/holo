use image::{Rgb, RgbImage};
use num_complex::Complex;
use opencv::core::Mat;

use crate::webcam::mat_to_rgb_image;

/// Schwellenwert für die Erkennung von Singularitäten.
/// Wenn die Real- oder Imaginärteile eines Ergebnisses diesen Wert überschreiten,
/// wird es als Singularität betrachtet.
pub const SINGULARITY_THRESHOLD: f64 = 1e6;

/// Standardpixel (Fallback), das verwendet wird, wenn eine Singularität auftritt
/// oder eine Abbildung fehlschlägt.
pub const FALLBACK_PIXEL: Rgb<u8> = Rgb([0, 0, 0]); // Schwarz

/// Eine Struktur zur Vorberechnung und Speicherung einer Lookup-Tabelle
/// für holomorphe Transformationen.
///
/// Die Lookup-Tabelle ordnet Pixelkoordinaten eines Ausgangsbildes neuen Koordinaten zu,
/// die durch eine mathematische holomorphe Funktion definiert werden.
///
/// # Felder
/// - `lookup` (`Vec<u32>`): Die vorab berechnete Tabelle, die jedem Pixel eine neue Position zuweist.
/// - `width` (`u32`): Die Breite des Bildes.
/// - `height` (`u32`): Die Höhe des Bildes.
pub struct HolomorphicLookup {
    pub lookup: Vec<u32>,
    pub width: u32,
    pub height: u32,
}

impl HolomorphicLookup {
    /// Erstellt eine neue holomorphe Lookup-Tabelle.
    ///
    /// Diese Funktion berechnet, wie Pixelkoordinaten durch eine holomorphe
    /// Transformation (definiert durch `f`) abgebildet werden. Die Ergebnisse
    /// werden in einer Tabelle gespeichert, um die Transformation effizient anzuwenden.
    ///
    /// # Parameter
    /// - `f` (`impl Fn(Complex<f64>) -> Complex<f64>`): Eine Funktion, die komplexe Zahlen
    ///   auf neue komplexe Zahlen abbildet. Dies definiert die Transformation.
    /// - `width` (`u32`): Die Breite des Bildes.
    /// - `height` (`u32`): Die Höhe des Bildes.
    ///
    /// # Rückgabewert
    /// Gibt eine neue Instanz von `HolomorphicLookup` zurück, die die vorab berechnete
    /// Transformation enthält.
    pub fn new(f: impl Fn(Complex<f64>) -> Complex<f64>, width: u32, height: u32) -> Self {
        let center_x = width as f64 / 2.0;
        let center_y = height as f64 / 2.0;

        // Initialisiere die Lookup-Tabelle
        let mut lookup = Vec::with_capacity((width * height) as usize);

        for y in 0..height {
            for x in 0..width {
                // Transformation der Pixelkoordinaten in komplexe Zahlen
                let cx = (x as f64 - center_x) / center_x;
                let cy = (y as f64 - center_y) / center_y;
                let complex_pos = Complex::new(cx, cy);

                // Anwenden der holomorphen Funktion
                let result = f(complex_pos);

                // Überprüfen auf Singularitäten
                if result.re.abs() > SINGULARITY_THRESHOLD
                    || result.im.abs() > SINGULARITY_THRESHOLD
                {
                    lookup.push((height - 1) * width); // Platzhalterindex für Singularitäten
                    continue;
                }

                // Rücktransformation der Ergebnisse in Bildkoordinaten
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
                let final_x = final_x.clamp(0.0, width as f64 - 1.0) as u32;
                let final_y = final_y.clamp(0.0, height as f64 - 1.0) as u32;

                // Calculate and store the index for the transformed coordinates
                let pixel_index = final_y * width + final_x;
                lookup.push(pixel_index);
            }
        }

        HolomorphicLookup {
            lookup,
            width,
            height,
        }
    }

    /// Gibt den transformierten Pixelindex für eine gegebene Position `(x, y)` zurück.
    ///
    /// # Parameter
    /// - `x` (`u32`): Die X-Koordinate des Pixels.
    /// - `y` (`u32`): Die Y-Koordinate des Pixels.
    ///
    /// # Rückgabewert
    /// Gibt den transformierten Index (`Option<u32>`) zurück oder `None`,
    /// falls die Koordinaten außerhalb der Bildgrenzen liegen.
    pub fn get(&self, x: u32, y: u32) -> Option<u32> {
        if x < self.width && y < self.height {
            let index = (y * self.width + x) as usize;
            self.lookup.get(index).cloned()
        } else {
            None
        }
    }

    /// Wendet die Lookup-Tabelle an, um ein Bild zu transformieren.
    ///
    /// # Parameter
    /// - `img` (`&RgbImage`): Das Eingabebild, das transformiert werden soll.
    ///
    /// # Rückgabewert
    /// Gibt das transformierte Bild (`Option<RgbImage>`) zurück oder `None`,
    /// falls die Transformation fehlschlägt.
    pub fn apply(&self, img: &RgbImage) -> Option<RgbImage> {
        let mut transformed_img = RgbImage::new(self.width, self.height);

        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(mapped_index) = self.get(x, y) {
                    let orig_x = mapped_index % self.width;
                    let orig_y = mapped_index / self.width;
                    let pixel = img.get_pixel(orig_x, orig_y);
                    transformed_img.put_pixel(x, y, *pixel);
                } else {
                    transformed_img.put_pixel(x, y, FALLBACK_PIXEL);
                }
            }
        }

        Some(transformed_img)
    }
}

/// Verarbeitet ein einzelnes Webcam-Frame und wendet eine holomorphe Transformation an.
///
/// # Parameter
/// - `lookup` (`&HolomorphicLookup`): Die vorab berechnete Lookup-Tabelle.
/// - `mat` (`&Mat`): Das Webcam-Frame im OpenCV-Mat-Format.
///
/// # Rückgabewert
/// Gibt das transformierte Bild (`Option<RgbImage>`) zurück oder `None`, falls ein Fehler auftritt.
pub fn process_frame(lookup: &HolomorphicLookup, mat: &Mat) -> Option<RgbImage> {
    let img = mat_to_rgb_image(mat)?;
    lookup.apply(&img)
}
