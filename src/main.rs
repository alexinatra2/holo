//! # Holomorphic Tinkering
//! Dieses Programm erlaubt es, holomorphe Transformationen auf Bildern oder Webcam-Frames
//! in Echtzeit anzuwenden. Benutzer können entweder eine Eingabedatei oder die Webcam
//! nutzen und eine mathematische Funktion als Transformation angeben.

mod cli;
mod display;
mod holo;
mod parsing;
mod webcam;

use chrono::Local;
use clap::Parser as ClapParser;
use cli::Cli;
use display::display_image;
use holo::{process_frame, HolomorphicLookup};
use image::RgbImage;
use minifb::{Key, Window, WindowOptions};
use opencv::videoio::{
    VideoCapture, VideoCaptureTrait, CAP_ANY, CAP_PROP_FRAME_HEIGHT, CAP_PROP_FRAME_WIDTH,
};
use parsing::Expr;
use std::path::Path;
use webcam::capture_frame;

/// Speichert ein transformiertes Bild im JPEG-Format in einem definierten Verzeichnis.
///
/// Diese Funktion generiert einen Dateinamen basierend auf dem ursprünglichen Bildnamen,
/// der angegebenen Transformationsfunktion und einem Zeitstempel. Der resultierende Dateiname
/// wird für eine sichere Speicherung angepasst.
///
/// # Parameter
/// - `image_path` (`&str`): Der Pfad zur ursprünglichen Bilddatei.
/// - `function_str` (`&str`): Die holomorphe Transformationsfunktion als String.
/// - `transformed_img` (`RgbImage`): Das transformierte Bild.
///
/// # Fehler
/// Gibt eine Fehlermeldung aus, wenn das Speichern des Bildes fehlschlägt.
fn save_transformed_image(image_path: &str, function_str: &str, transformed_img: RgbImage) {
    // Extrahiere den Dateinamen aus dem Pfad
    let input_filename = Path::new(image_path)
        .file_stem()
        .expect("Failed to extract file name")
        .to_str()
        .expect("Failed to convert to string");

    let timestamp = Local::now().format("%Y%m%d%H%M%S").to_string();

    // Ersetze "/" durch "div" für einen sicheren Dateinamen
    let sanitized_function_str: String = function_str
        .replace("/", "div")
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();

    // Generiere den Dateinamen
    let output_filename = format!(
        "./images/output/{}_{}_{}.jpeg",
        input_filename, sanitized_function_str, timestamp
    );

    // Speichere das Bild
    transformed_img
        .save(&output_filename)
        .expect("Failed to save image");

    println!("Image saved as: {}", output_filename);
}

/// Der Haupteinstiegspunkt des Programms.
///
/// Parst die Kommandozeilenargumente, lädt ein Bild oder öffnet die Webcam, wendet
/// die angegebene holomorphe Funktion an und zeigt das Ergebnis an oder speichert es.
///
/// # Rückgabewert
/// Gibt `Ok(())` zurück, wenn das Programm erfolgreich ausgeführt wurde,
/// oder einen Fehler, falls etwas schiefgeht.
///
/// # Fehler
/// - Gibt Fehler zurück, wenn die Eingabedaten ungültig sind.
/// - Kann Fehler ausgeben, wenn die Webcam nicht verfügbar ist oder die Transformation fehlschlägt.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    let input = &args.function;
    let expression = Expr::parse(input)?;
    let holomorphic_fn = expression.get_closure();

    match args.image {
        Some(file_path) => {
            let image_file_path = Path::new(&file_path);
            let image_file_name = image_file_path.file_name().unwrap();
            let image_path_string =
                format!("./images/output/{}", image_file_name.to_string_lossy());
            let image_path: &str = &image_path_string;

            // Lade das Bild
            let img = image::open(image_file_path)
                .expect("Failed to load image")
                .to_rgb8();

            let (width, height) = img.dimensions();
            let lookup = HolomorphicLookup::new(holomorphic_fn, width, height);

            if let Some(transformed_img) = lookup.apply(&img) {
                save_transformed_image(image_path, input, transformed_img);
            } else {
                eprint!("transforming image unsuccessful");
            }
        }
        None => {
            // Bestimme Breite und Höhe basierend auf Auflösung oder Dimensionen
            let (width, height) = if let Some(res) = args.resolution {
                res.to_dimensions()
            } else {
                args.dimensions.unwrap_or((640, 480)) // Standard-Dimensionen
            };
            let lookup = HolomorphicLookup::new(holomorphic_fn, width, height);
            let mut cap = VideoCapture::new(0, CAP_ANY)?; // 0 ist die Standardkamera
            cap.set(CAP_PROP_FRAME_WIDTH, width as f64)?;
            cap.set(CAP_PROP_FRAME_HEIGHT, height as f64)?;

            // Erstelle ein Anzeigefenster
            let mut window = Window::new(
                "Holomorphic Webcam",
                width as usize,
                height as usize,
                WindowOptions::default(),
            )
            .expect("Failed to create window");
            while window.is_open() && !window.is_key_down(Key::Escape) {
                // Frame von der Webcam erfassen
                if let Some(frame) = capture_frame(&mut cap) {
                    // Transformation anwenden
                    if let Some(transformed_image) = process_frame(&lookup, &frame) {
                        // Transformiertes Bild anzeigen
                        display_image(&mut window, &transformed_image);
                    }
                }
            }
        }
    }

    Ok(())
}
