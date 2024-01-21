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

    if let Some(wallpaper_dir) = matches.get_one::<PathBuf>("dir") {
        if let Ok(selected) = select_wallpaper(wallpaper_dir) {
            match change_wallpaper(&selected) {
                Ok(_) => println!("New wallpaper set: {:?}", selected),
                Err(err) => println!("Unable to set wallpaper: {:?}", err),
            }
        } else {
            println!("No suitable image found")
        }
    }
}
