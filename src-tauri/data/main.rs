// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use serde_json::Error;
use tauri::{utils::config::WindowConfig, Menu, MenuItem, Submenu, WindowBuilder};

fn json_to_window_config(json: &str) -> Result<WindowConfig, Error> {
    serde_json::from_str(json)
}

fn main() {
    let edit_menu = Submenu::new(
        "Edit",
        Menu::new()
            .add_native_item(MenuItem::Undo)
            .add_native_item(MenuItem::Redo)
            .add_native_item(MenuItem::Copy)
            .add_native_item(MenuItem::Cut)
            .add_native_item(MenuItem::Paste)
            .add_native_item(MenuItem::SelectAll)
            .add_native_item(MenuItem::CloseWindow)
            .add_native_item(MenuItem::Quit),
    );
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle();
            let json = r#"{"label":"submain","url":"https://juejin.cn/"}"#;
            match json_to_window_config(json) {
                Ok(config) => {
                    println!("Parsed WindowConfig: {:?}", config);
                    let _main_window = WindowBuilder::from_config(&app_handle, config)
                        .build()
                        .unwrap();
                }
                Err(err) => {
                    eprintln!("Failed to parse JSON: {}", err);
                }
            }
            Ok(())
        })
        .menu(Menu::new().add_submenu(edit_menu))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
