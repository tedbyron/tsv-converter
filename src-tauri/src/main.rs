#![warn(
    clippy::all,
    clippy::cargo,
    clippy::nursery,
    clippy::pedantic,
    rust_2018_idioms
)]
#![windows_subsystem = "windows"]

use std::path::Path;

use time::OffsetDateTime;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![file_metadata])
        .run(tauri::generate_context!())
        .expect("Error while running the application");
}

/// Serializable struct to pass file metadata to JavaScript.
#[derive(serde::Serialize)]
struct Metadata {
    name: Option<String>,
    mimes: Vec<String>,
    len: Option<u64>,
    #[serde(with = "time::serde::timestamp::option")]
    created: Option<OffsetDateTime>,
    #[serde(with = "time::serde::timestamp::option")]
    modified: Option<OffsetDateTime>,
}

/// Retrieve file metadata from a path.
#[tauri::command]
fn file_metadata(path: String) -> Metadata {
    let path = Path::new(&path);
    let mimes = mime_guess::from_path(path)
        .iter_raw()
        .map(String::from)
        .collect();
    let name = path.file_name().map(|s| s.to_string_lossy().to_string());

    let (mut len, mut created, mut modified) = (None, None, None);
    if let Ok(meta) = path.metadata() {
        len = Some(meta.len());
        created = meta.created().ok().map(OffsetDateTime::from);
        modified = meta.modified().ok().map(OffsetDateTime::from);
    }

    Metadata {
        name,
        mimes,
        len,
        created,
        modified,
    }
}
