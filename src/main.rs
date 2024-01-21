use clap::{arg, command, value_parser};
use std::path::PathBuf;

fn main() {
    let matches = command!()
        .arg(
            arg!(-d --dir <DIR>)
                .required(true)
                .value_parser(value_parser!(PathBuf)),
        )
        .get_matches();

    if let Some(wallpaper_dir) = matches.get_one::<PathBuf>("dir") {
        println!("Wallpaper {}", wallpaper_dir.display());
        for file_name in wallpaper_dir
            .read_dir()
            .expect("Could not read this directory")
        {
            if let Ok(file_name) = file_name {
                println!("{:?}", file_name.path());
            }
        }
    }
}
