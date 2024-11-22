use clap::{command, Parser, ValueEnum};

/// Benutzerdefinierte Parser-Funktion, um Dimensionen im Format `width,height` zu parsen.
///
/// # Parameter
/// - `s` (`&str`): Die Eingabe im Format `width,height`.
///
/// # Rückgabewert
/// Gibt entweder die geparsten Dimensionen als Tupel `(u32, u32)` zurück oder
/// eine Fehlermeldung (`String`), falls das Format ungültig ist.
///
/// # Fehler
/// - Gibt einen Fehler zurück, wenn die Eingabe nicht genau zwei Werte enthält,
///   getrennt durch ein Komma.
/// - Gibt einen Fehler zurück, wenn `width` oder `height` keine gültigen Ganzzahlen sind.
fn parse_dimensions(s: &str) -> Result<(u32, u32), String> {
    let parts: Vec<&str> = s.split(',').collect();
    if parts.len() != 2 {
        return Err(String::from("Dimensions must be in format width,height"));
    }
    let width = parts[0].parse::<u32>().map_err(|_| "Invalid width")?;
    let height = parts[1].parse::<u32>().map_err(|_| "Invalid height")?;
    Ok((width, height))
}

/// Aufzählung gängiger Bildschirmauflösungen.
///
/// Diese Enum definiert verschiedene vordefinierte Bildschirmauflösungen und
/// kann in Verbindung mit der `--resolution`-Option verwendet werden.
#[derive(ValueEnum, Clone, Debug)]
pub enum Resolution {
    Hd,      // 1280x720
    FullHd,  // 1920x1080
    Uhd,     // 3840x2160
    Qhd,     // 2560x1440
    Wqhd,    // 2560x1600
    FourK,   // 3840x2160
    EightK,  // 7680x4320
    Sd,      // 640x480
    Retina,  // 2048x1536
    Svga,    // 800x600
    Xga,     // 1024x768
    Wxga,    // 1280x800
    HdReady, // 1366x768
    Wvga,    // 800x480
    Qvga,    // 320x240
    Cga,     // 640x200
}

impl Resolution {
    /// Gibt die Breite und Höhe der Auflösung als Tupel `(u32, u32)` zurück.
    ///
    /// # Rückgabewert
    /// Ein Tupel, das die Dimensionen der jeweiligen Auflösung beschreibt.
    pub fn to_dimensions(&self) -> (u32, u32) {
        match self {
            Resolution::Hd => (1280, 720),
            Resolution::FullHd => (1920, 1080),
            Resolution::Uhd => (3840, 2160),
            Resolution::Qhd => (2560, 1440),
            Resolution::Wqhd => (2560, 1600),
            Resolution::FourK => (3840, 2160),
            Resolution::EightK => (7680, 4320),
            Resolution::Sd => (640, 480),
            Resolution::Retina => (2048, 1536),
            Resolution::Svga => (800, 600),
            Resolution::Xga => (1024, 768),
            Resolution::Wxga => (1280, 800),
            Resolution::HdReady => (1366, 768),
            Resolution::Wvga => (800, 480),
            Resolution::Qvga => (320, 240),
            Resolution::Cga => (640, 200),
        }
    }
}

/// Definiert die Kommandozeilenargumente für das Programm.
///
/// Diese Struktur verwendet die `clap`-Bibliothek, um Argumente aus der Kommandozeile zu parsen.
/// Die Optionen ermöglichen die Angabe von Transformationen, Bilddateien und Auflösungen.
///
/// # Felder
/// - `function` (`String`): Die mathematische Funktion, die auf die Bilddaten angewendet wird.
/// - `image` (`Option<String>`): Der Pfad zur Bilddatei, die verarbeitet werden soll. Wenn keine Bilddatei angegeben wird, wird die Webcam verwendet.
/// - `resolution` (`Option<Resolution>`): Eine vordefinierte Auflösung, die benutzerdefinierte Dimensionen überschreibt.
/// - `dimensions` (`Option<(u32, u32)>`): Benutzerdefinierte Dimensionen im Format `width,height`.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// The function string to apply
    #[arg(
        value_name = "FUNCTION",
        help = "Function to apply to the file contents"
    )]
    pub function: String,

    /// The filename to process (supports file completion in some shells)
    #[arg(
        value_name = "IMAGE_FILENAME",
        help = "Path to the file to process",
        value_hint = clap::ValueHint::FilePath,
        short,
        long
    )]
    pub image: Option<String>,

    /// Resolution preset, overriding custom dimensions if specified
    #[arg(short, long, value_enum)]
    pub resolution: Option<Resolution>,

    /// Custom dimensions in the format width,height
    #[arg(short, long, value_parser = parse_dimensions)]
    pub dimensions: Option<(u32, u32)>,
}
