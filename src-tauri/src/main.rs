#![warn(
    clippy::all,
    clippy::cargo,
    clippy::nursery,
    clippy::pedantic,
    rust_2018_idioms
)]
#![windows_subsystem = "windows"]

use std::ffi::OsString;
use std::path::Path;
use std::time::SystemTime;

#[derive(serde::Serialize)]
struct VideoMetadata {
    name: OsString,
    mimes: Vec<String>,
    len: u64,
    created: Option<SystemTime>,
    modified: Option<SystemTime>,
}

fn main() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn file_metadata(path: String) -> Result<VideoMetadata, String> {
    let path = Path::new(&path);
    let mimes = mime_guess::from_path(path)
        .iter_raw()
        .map(String::from)
        .collect();
    let name = path
        .file_name()
        .ok_or_else(|| "Failed to get file name.")?
        .to_os_string();
    let meta = path
        .metadata()
        .map_err(|_| "Failed to get file metadata.")?;

    Ok(VideoMetadata {
        name,
        mimes,
        len: meta.len(),
        created: meta.created().ok(),
        modified: meta.modified().ok(),
    })
}
