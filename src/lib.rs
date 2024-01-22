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
    InvalidPath,
}

const IMAGE_EXTENSIONS: [&str; 2] = ["jpg", "png"];

fn is_image_ext(ext: &OsStr) -> bool {
    IMAGE_EXTENSIONS
        .iter()
        .map(OsString::from)
        .collect::<Vec<OsString>>()
        .contains(&ext.to_ascii_lowercase())
}

fn matches_image_path(entry: DirEntry) -> Option<PathBuf> {
    let path = entry.path();
    match path.extension() {
        Some(ext) if is_image_ext(ext) => Some(path),
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

fn select_wallpaper(wallpaper_dir: &PathBuf) -> Result<String, WallpaperError> {
    let entries = wallpaper_dir
        .read_dir()
        .map_err(|_| WallpaperError::DirectoryNotFound)?;

    let paths: Vec<PathBuf> = entries
        .flat_map(|e| e)
        .filter_map(matches_image_path)
        .collect();

    let file_name = paths
        .choose(&mut thread_rng())
        .ok_or(WallpaperError::ImageNotFound)?
        .to_str()
        .ok_or(WallpaperError::InvalidPath)?;

    Ok(file_name.to_string())
}

pub fn change_wallpaper(wallpaper_dir: &PathBuf) -> Result<String, WallpaperError> {
    let file_name = select_wallpaper(wallpaper_dir)?;

    for (schema, key) in vec![
        ("org.gnome.desktop.background", "picture-uri"),
        ("org.gnome.desktop.background", "picture-uri-dark"),
        ("org.gnome.desktop.screensaver", "picture-uri"),
    ] {
        gsettings_set(schema, key, &file_name)?;
    }
    Ok(file_name)
}
