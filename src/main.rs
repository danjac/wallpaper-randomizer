use clap::{arg, command, value_parser};

use std::path::PathBuf;
use wallpaper_randomizer::{change_wallpaper, select_wallpaper};

fn main() {
    let matches = command!()
        .arg(
            arg!(-d --dir <DIR>)
                .required(true)
                .value_parser(value_parser!(PathBuf)),
        )
        .get_matches();

    if let Some(dir) = matches.get_one::<PathBuf>("dir") {
        if let Ok(path) = select_wallpaper(dir) {
            match change_wallpaper(&path) {
                Ok(path) => println!("New wallpaper set: {:?}", path),
                Err(err) => println!("Unable to set wallpaper: {:?}", err),
            }
        } else {
            println!("No suitable image found")
        }
    }
}
