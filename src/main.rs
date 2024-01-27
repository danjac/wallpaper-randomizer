use clap::Parser;

use std::path::PathBuf;
use wallpaper_randomizer::change_wallpaper;

/// Randomly select a Gnome wallpaper.
///
/// Only JPEG and PNG images are supported.
#[derive(Parser)]
#[command(version)]
struct Cli {
    dir: PathBuf,
}

fn main() {
    let cli = Cli::parse();
    match change_wallpaper(&cli.dir) {
        Ok(path) => println!("New wallpaper set: {path}"),
        Err(err) => eprintln!("Cannot set wallpaper: {err}"),
    }
}
