#![warn(clippy::all, clippy::nursery, rust_2018_idioms)]
#![windows_subsystem = "windows"]

mod commands;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::metadata,
            commands::watch,
            commands::output_name,
            commands::convert,
        ])
        .run(tauri::generate_context!())
        .expect("Error while running the application");
}
