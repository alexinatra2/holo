use clap::{command, Parser};

/// Custom parser function to parse dimensions in the format `width,height`.
fn parse_dimensions(s: &str) -> Result<(u32, u32), String> {
    let parts: Vec<&str> = s.split(',').collect();
    if parts.len() != 2 {
        return Err(String::from("Dimensions must be in format width,height"));
    }

    let width = parts[0].parse::<u32>().map_err(|_| "Invalid width")?;
    let height = parts[1].parse::<u32>().map_err(|_| "Invalid height")?;

    Ok((width, height))
}

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

    /// Dimensions in the format width,height
    #[arg(short, long, value_parser = parse_dimensions, default_value = "640,480")]
    pub dimensions: Option<(u32, u32)>,
}
