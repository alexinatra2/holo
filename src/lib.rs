/// Dieses Modul enthält die Hauptfunktionalität zur Transformation von Bildern
/// mit holomorphen Funktionen. Es nutzt verschiedene Submodule, um Parsing,
/// Transformation und Bildverarbeitung zu implementieren.

pub mod display;
pub mod holo;
pub mod parsing;
pub mod webcam;

use std::io::Cursor;

use holo::HolomorphicLookup;
use image::{codecs::png::PngEncoder, ExtendedColorType, ImageEncoder, RgbImage};
use parsing::Expr;
use wasm_bindgen::prelude::wasm_bindgen;

/// Transformiert ein Eingabebild basierend auf einer angegebenen holomorphen Funktion.
///
/// Diese Funktion nimmt ein Bild als Byte-Array und eine mathematische Funktion in Stringform.
/// Sie wendet die Funktion auf jedes Pixel des Bildes an und gibt das transformierte Bild
/// als PNG-kodierte Byte-Daten zurück.
///
/// # Parameter
/// - `image_data` (`Vec<u8>`): Die Rohdaten des Eingabebildes im RGB-Format.
/// - `func_str` (`String`): Die mathematische Funktion, die auf das Bild angewendet wird.
///   Diese Funktion muss holomorph sein und in einer unterstützten Syntax vorliegen.
/// - `width` (`u32`): Die Breite des Eingabebildes in Pixeln.
/// - `height` (`u32`): Die Höhe des Eingabebildes in Pixeln.
///
/// # Rückgabewert
/// Gibt optional die transformierten Bilddaten zurück (`Option<Vec<u8>>`).
/// Wenn ein Fehler auftritt (z. B. beim Parsen der Funktion oder der Bildmanipulation),
/// wird `None` zurückgegeben.
///
/// # Fehler
/// - Wenn die Eingabe-Bilddaten ungültig sind.
/// - Wenn die Funktion nicht korrekt geparst werden kann.
/// - Wenn ein Fehler beim Kodieren der PNG-Daten auftritt.
#[wasm_bindgen]
pub fn transform_image(
    image_data: Vec<u8>,
    func_str: String,
    width: u32,
    height: u32,
) -> Option<Vec<u8>> {
    // Parse die holomorphe Funktion aus dem Eingabestring
    let parsed_function = Expr::parse(&func_str).ok();
    let holomorphic_fn = parsed_function?.get_closure();

    // Konvertiere die Eingabe-Bilddaten in ein Bildobjekt
    let img = RgbImage::from_raw(width, height, image_data).unwrap();
    let (width, height) = img.dimensions();

    // Wende die holomorphe Funktion auf das Bild an
    let lookup = HolomorphicLookup::new(holomorphic_fn, width, height);
    let transformed_img = lookup.apply(&img)?;

    // Konvertiere das transformierte Bild zurück in einen Byte-Vektor zur Rückgabe
    let mut transformed_data = Vec::new();
    let mut cursor = Cursor::new(&mut transformed_data);
    // Erstelle einen PNG-Encoder und schreibe die Bilddaten in den Cursor
    let encoder = PngEncoder::new(&mut cursor);
    encoder
        .write_image(
            transformed_img.as_raw(),
            transformed_img.width(),
            transformed_img.height(),
            ExtendedColorType::Rgb8,
        )
        .expect("Failed to encode PNG image");

    Some(transformed_data)
}
