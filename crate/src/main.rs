#![warn(clippy::all, clippy::cargo, clippy::nursery, rust_2018_idioms)]
#![windows_subsystem = "windows"]

mod command;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            command::metadata,
            command::watch,
            command::output_name,
            command::convert,
        ])
        .run(tauri::generate_context!())
        .expect("Error while running the application");
}
