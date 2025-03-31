use rand::seq::SliceRandom;
use rand::thread_rng;
use std::ffi::OsStr;
use std::fmt;
use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

#[derive(Debug)]
pub enum WallpaperError {
    CommandIo(io::Error),
    CommandFailed(Vec<u8>),
    DirectoryNotFound,
    ImageNotFound,
    InvalidPath,
}

impl fmt::Display for WallpaperError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::CommandIo(err) => write!(f, "error trying to set GNOME setting: {err}"),
            Self::CommandFailed(err) => write!(
                f,
                "error trying to set GNOME setting: {}",
                String::from_utf8_lossy(err)
            ),
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

fn set_picture_uri(schema: &str, key: &str, file_name: &str) -> Result<(), WallpaperError> {
    gsettings_set(schema, key, &format!("file://{file_name}"))
}

fn gsettings_set(schema: &str, key: &str, option: &str) -> Result<(), WallpaperError> {
    let output = Command::new("gsettings")
        .arg("set")
        .arg(schema)
        .arg(key)
        .arg(option)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .output()
        .map_err(WallpaperError::CommandIo)?;

    if !output.status.success() {
        return Err(WallpaperError::CommandFailed(output.stderr));
    }
    Ok(())
}


fn select_wallpaper(wallpaper_dir: &Path) -> Result<String, WallpaperError> {
    // select all PNG and JPEG files in directory
    let paths: Vec<PathBuf> = wallpaper_dir
        .read_dir()
        .map_err(|_| WallpaperError::DirectoryNotFound)?
        .flatten()
        .map(|e| e.path())
        .filter(|path| path.extension().is_some_and(is_image_ext))
        .collect();

    // choose one path at random
    let file_name = paths
        .choose(&mut thread_rng())
        .ok_or(WallpaperError::ImageNotFound)?
        .to_str()
        .ok_or(WallpaperError::InvalidPath)?;

    Ok(file_name.to_string())
}

pub fn change_wallpaper(wallpaper_dir: &Path, option: &str) -> Result<String, WallpaperError> {
    // select a random wallpaper path and apply Gnome desktop settings
    let file_name = select_wallpaper(wallpaper_dir)?;

    for (schema, key) in [
        ("org.gnome.desktop.background", "picture-uri"),
        ("org.gnome.desktop.background", "picture-uri-dark"),
        ("org.gnome.desktop.screensaver", "picture-uri"),
    ] {
        set_picture_uri(schema, key, &file_name)?;
    }
    for (schema, key) in [
        ("org.gnome.desktop.background", "picture-options"),
        ("org.gnome.desktop.screensaver", "picture-options"),
    ] {
        gsettings_set(schema, key, option)?;
    }
    Ok(file_name)
}
