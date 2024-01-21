use clap::{arg, command, value_parser};

use rand::seq::SliceRandom;
use rand::thread_rng;
use std::ffi::{OsStr, OsString};
use std::fs::DirEntry;
use std::path::PathBuf;
use std::process::Command;

#[derive(PartialEq, Debug)]
enum Error {
    CommandError,
    DirectoryNotFound,
    ImageNotFound,
}

fn is_image_ext(ext: &OsStr) -> bool {
    ["jpg", "png"]
        .iter()
        .map(OsString::from)
        .collect::<Vec<OsString>>()
        .contains(&ext.to_ascii_lowercase())
}

fn matches_image_path(result: Result<DirEntry, std::io::Error>) -> Option<PathBuf> {
    match result {
        Ok(entry) => match entry.path() {
            path => match path.extension() {
                Some(ext) if is_image_ext(ext) => Some(path),
                _ => None,
            },
        },
        _ => None,
    }
}

fn select_wallpaper(wallpaper_dir: &PathBuf) -> Result<PathBuf, Error> {
    if let Ok(wallpaper_dir) = wallpaper_dir.read_dir() {
        let file_names: Vec<PathBuf> = wallpaper_dir.filter_map(matches_image_path).collect();
        let mut rng = thread_rng();

        if let Some(selected) = file_names.choose(&mut rng) {
            Ok(selected.clone())
        } else {
            Err(Error::ImageNotFound)
        }
    } else {
        Err(Error::DirectoryNotFound)
    }
}

fn change_wallpaper(file_name: &PathBuf) -> Result<(), Error> {
    if let Some(file_name) = file_name.to_str() {
        for (schema, key) in vec![
            ("org.gnome.desktop.background", "picture-uri"),
            ("org.gnome.desktop.background", "picture-uri-dark"),
            ("org.gnome.desktop.screensaver", "picture-uri"),
        ] {
            if let Err(_) = Command::new("gsettings")
                .arg("set")
                .arg(schema)
                .arg(key)
                .arg(format!("file://{}", file_name))
                .output()
            {
                return Err(Error::CommandError);
            }
        }
        Ok(())
    } else {
        Err(Error::ImageNotFound)
    }
}

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
                _ => println!("Unable to set wallpaper"),
            }
        } else {
            println!("No suitable image found")
        }
    }
}
