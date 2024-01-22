use rand::seq::SliceRandom;
use rand::thread_rng;
use std::ffi::{OsStr, OsString};
use std::fs::DirEntry;
use std::io::Error;
use std::path::PathBuf;
use std::process::Command;

#[derive(Debug)]
pub enum WallpaperError {
    CommandError(Error),
    DirectoryNotFound,
    ImageNotFound,
}

const IMAGE_EXTENSIONS: [&str; 2] = ["jpg", "png"];

fn is_image_ext(ext: &OsStr) -> bool {
    IMAGE_EXTENSIONS
        .iter()
        .map(OsString::from)
        .collect::<Vec<OsString>>()
        .contains(&ext.to_ascii_lowercase())
}

fn matches_image_path(result: Result<DirEntry, Error>) -> Option<PathBuf> {
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

fn gsettings_set(schema: &str, key: &str, file_name: &str) -> Result<(), WallpaperError> {
    Command::new("gsettings")
        .arg("set")
        .arg(schema)
        .arg(key)
        .arg(format!("file://{}", file_name))
        .output()
        .map_err(|e| WallpaperError::CommandError(e))?;
    Ok(())
}

pub fn select_wallpaper(wallpaper_dir: &PathBuf) -> Result<PathBuf, WallpaperError> {
    let dir = wallpaper_dir
        .read_dir()
        .map_err(|_| WallpaperError::DirectoryNotFound)?;

    let paths: Vec<PathBuf> = dir.filter_map(matches_image_path).collect();

    match paths.choose(&mut thread_rng()) {
        Some(path) => Ok(path.clone()),
        _ => Err(WallpaperError::ImageNotFound),
    }
}

pub fn change_wallpaper(path: &PathBuf) -> Result<&PathBuf, WallpaperError> {
    let file_name = path.to_str().ok_or(WallpaperError::ImageNotFound)?;

    for (schema, key) in vec![
        ("org.gnome.desktop.background", "picture-uri"),
        ("org.gnome.desktop.background", "picture-uri-dark"),
        ("org.gnome.desktop.screensaver", "picture-uri"),
    ] {
        gsettings_set(schema, key, file_name)?
    }
    Ok(path)
}
