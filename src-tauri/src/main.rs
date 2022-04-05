#![warn(
    clippy::all,
    clippy::cargo,
    clippy::nursery,
    clippy::pedantic,
    rust_2018_idioms
)]
#![windows_subsystem = "windows"]

use std::path::PathBuf;
use std::sync::Mutex;

use tauri::Manager;

mod metadata;

type VideoPath = Mutex<Option<PathBuf>>;

fn main() {
    tauri::Builder::default()
        .manage(VideoPath::default())
        .setup(|app| {
            let handle = app.handle();

            app.listen_global("video-path", move |event| {
                if let Some(path) = event.payload() {
                    let path = PathBuf::from(path);
                    handle
                        .emit_all("video-metadata", metadata::get(&path))
                        .unwrap();
                    *handle.state::<VideoPath>().lock().unwrap() = Some(path);
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("Error while running the application");
}
