use clap::{arg, command, value_parser};

use rand::seq::SliceRandom;
use rand::thread_rng;
use std::ffi::OsStr;
use std::fs::DirEntry;
use std::path::PathBuf;
use std::process::Command;

#[derive(PartialEq, Debug)]
enum Error {
    DirectoryNotFound,
    ImageNotFound,
}

fn is_image_ext(ext: &OsStr) -> bool {
    ["jpg", "png"]
        .iter()
        .map(OsStr::new)
        .collect::<Vec<&OsStr>>()
        .contains(&ext)
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

fn change_wallpaper(file_name: &PathBuf) {
    if let Some(file_name) = file_name.to_str() {
        for setting in vec![
            ("org.gnome.desktop.background", "picture-uri"),
            ("org.gnome.desktop.background", "picture-uri-dark"),
            ("org.gnome.desktop.screensaver", "picture-uri"),
        ] {
            Command::new("gsettings")
                .arg("set")
                .arg(setting.0)
                .arg(setting.1)
                .arg(format!("file://{}", file_name))
                .spawn()
                .expect("Error running setting");
        }
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
            println!("selected path:{:?}", selected);
            change_wallpaper(&selected);
        } else {
            println!("No suitable image found")
        }
    }
}
