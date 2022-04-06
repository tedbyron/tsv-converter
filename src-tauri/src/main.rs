#![warn(
    clippy::all,
    clippy::cargo,
    clippy::nursery,
    clippy::pedantic,
    rust_2018_idioms
)]
#![windows_subsystem = "windows"]

mod command;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![command::metadata])
        .run(tauri::generate_context!())
        .expect("Error while running the application");
}
