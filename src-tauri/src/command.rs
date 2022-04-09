//! Tauri commands.

use std::path::{Path, PathBuf};

use notify::{Config, EventKind, RecursiveMode, Watcher};
use time::OffsetDateTime;

// Corresponds to the `Metadata` type in `src/lib/FileStatTable.svelte`.
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    name: Option<String>,
    file_stem: Option<String>,
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

    let name = path.file_name().map(|s| s.to_string_lossy().to_string());

    // The file stem will be used to name the converted video, so we don't want invalid UTF-8.
    let file_stem = path
        .file_stem()
        .map(|s| s.to_str().map(|s| s.to_string()))
        .flatten();

    let mimes = mime_guess::from_path(path)
        .iter_raw()
        .map(String::from)
        .collect();

    let (mut len, mut created, mut modified) = (None, None, None);
    if let Ok(meta) = path.metadata() {
        len = Some(meta.len());
        created = meta.created().ok().map(OffsetDateTime::from);
        modified = meta.modified().ok().map(OffsetDateTime::from);
    }

    Metadata {
        name,
        file_stem,
        mimes,
        len,
        created,
        modified,
    }
}

#[tauri::command]
pub async fn watch(path: String, window: tauri::Window) {
    let path = PathBuf::from(path);
    let (tx, mut rx) = tauri::async_runtime::channel(1);

    let mut watcher =
        notify::recommended_watcher(move |res: Result<notify::Event, notify::Error>| {
            match res {
                Ok(event) => match event.kind {
                    EventKind::Modify(_) | EventKind::Remove(_) => {
                        tx.blocking_send("fs-change").unwrap()
                    }
                    _ => (),
                },
                Err(err) => eprintln!("Path watch error: {err:?}"),
            };
        })
        .unwrap();
    watcher.configure(Config::PreciseEvents(true)).unwrap();
    watcher.watch(&path, RecursiveMode::NonRecursive).unwrap();

    while let Some(msg) = rx.recv().await {
        window.emit(msg, ()).unwrap();
        watcher.unwatch(&path).unwrap();
        break;
    }
}
