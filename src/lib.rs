use rand::seq::SliceRandom;
use rand::thread_rng;
use std::ffi::OsStr;
use std::fmt;
use std::io::Error;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug)]
pub enum WallpaperError {
    CommandError(Error),
    DirectoryNotFound,
    ImageNotFound,
    InvalidPath,
}

impl fmt::Display for WallpaperError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::CommandError(err) => write!(f, "error trying to set GNOME setting: {err}"),
            Self::DirectoryNotFound => write!(f, "directory not found"),
            Self::ImageNotFound => write!(f, "unable to find a JPEG or PNG"),
            Self::InvalidPath => write!(f, "does not appear to be valid path"),
        }
    }
}

const IMAGE_EXTENSIONS: [&str; 3] = ["jpg", "jpeg", "png"];

fn is_image_ext(ext: &OsStr) -> bool {
    ext.to_str()
        .is_some_and(|ext| IMAGE_EXTENSIONS.contains(&ext.to_ascii_lowercase().as_str()))
}

fn matches_image_path(path: PathBuf) -> Option<PathBuf> {
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
        .arg(format!("file://{file_name}"))
        .output()
        .map_err(WallpaperError::CommandError)?;
    Ok(())
}

fn select_wallpaper(wallpaper_dir: &Path) -> Result<String, WallpaperError> {
    // select all PNG and JPEG files in directory
    let paths: Vec<PathBuf> = wallpaper_dir
        .read_dir()
        .map_err(|_| WallpaperError::DirectoryNotFound)?
        .flatten()
        .map(|e| e.path())
        .filter_map(matches_image_path)
        .collect();

    // choose one path at random
    let file_name = paths
        .choose(&mut thread_rng())
        .ok_or(WallpaperError::ImageNotFound)?
        .to_str()
        .ok_or(WallpaperError::InvalidPath)?;

    Ok(file_name.to_string())
}

pub fn change_wallpaper(wallpaper_dir: &Path) -> Result<String, WallpaperError> {
    // select a random wallpaper path and apply Gnome desktop settings
    let file_name = select_wallpaper(wallpaper_dir)?;

    for (schema, key) in [
        ("org.gnome.desktop.background", "picture-uri"),
        ("org.gnome.desktop.background", "picture-uri-dark"),
        ("org.gnome.desktop.screensaver", "picture-uri"),
    ] {
        gsettings_set(schema, key, &file_name)?;
    }
    Ok(file_name)
}
