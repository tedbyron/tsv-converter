#![warn(clippy::all, clippy::nursery, rust_2018_idioms)]
#![windows_subsystem = "windows"]

mod commands;
#[cfg(target_os = "macos")]
mod macos;

fn main() {
    tauri::Builder::default()
        // .setup(|app| {
        //     #[cfg(target_os = "macos")]
        //     {
        //         use tauri::Manager;

        //         use macos::WindowExt;

        //         app.get_window("main").unwrap().set_titlebar_transparent();
        //     }

        //     Ok(())
        // })
        .invoke_handler(tauri::generate_handler![
            commands::metadata,
            commands::watch,
            commands::output_name,
            commands::convert,
        ])
        .run(tauri::generate_context!())
        .expect("Error running the application");
}
