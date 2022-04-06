//! Tauri commands.

use std::path::Path;
use std::sync::{Arc, Mutex};

use time::OffsetDateTime;

// Corresponds to the `Metadata` type in `src/lib/FileStatTable.svelte`.
#[derive(serde::Serialize)]
pub struct Metadata {
    name: Option<String>,
    mimes: Vec<String>,
    len: Option<u64>,
    #[serde(with = "time::serde::timestamp::option")]
    created: Option<OffsetDateTime>,
    #[serde(with = "time::serde::timestamp::option")]
    modified: Option<OffsetDateTime>,
}

#[tauri::command]
pub async fn metadata(path: String) -> Metadata {
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

#[tauri::command]
pub async fn watch<R: tauri::Runtime>(app: tauri::AppHandle<R>) -> Result<(), String> {
    let (tx, rx) = tauri::async_runtime::channel(4);
    let watcher = notify::recommended_watcher(todo!()).map_err(|e| e.to_string())?;
    let watcher = Arc::new(Mutex::new(watcher));

    Ok(())
}
