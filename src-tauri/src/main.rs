#![forbid(unsafe_code)]
#![warn(
    clippy::all,
    clippy::cargo,
    clippy::nursery,
    clippy::pedantic,
    rust_2018_idioms
)]
#![windows_subsystem = "windows"]

use std::fs;
use std::io;
use std::path::Path;
use std::time::SystemTime;

#[derive(serde::Serialize)]
struct VideoMetadata {
    name: String,
    size: u64,
    modified: Option<SystemTime>,
}

fn main() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn file_metadata<P>(path: P) -> io::Result<VideoMetadata>
where
    P: AsRef<Path>,
{
    let meta = fs::metadata(path)?;

    Ok(VideoMetadata {
        name: todo!(),
        size: meta.len(),
        modified: meta.modified().ok(),
    })
}
