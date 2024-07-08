// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::config::init_config;
use crate::shortcut::register_shortcut;
use log::{error, info};
use once_cell::sync::OnceCell;
use tauri::Manager;
use tauri_plugin_log::LogTarget;
mod cmd;
mod config;
mod path;
mod profile;
mod shortcut;
mod system_ocr;
mod task;
mod translate;
mod window;
mod monitor;

pub static APP: OnceCell<tauri::AppHandle> = OnceCell::new();
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets([LogTarget::LogDir, LogTarget::Stdout, LogTarget::Webview])
                .build(),
        )
        .setup(|app| {
            #[cfg(target_os = "macos")]
            {
                // hide taskbar icon on macOS
                app.set_activation_policy(tauri::ActivationPolicy::Accessory);
            }

            // Global AppHandle
            APP.set(app.app_handle().clone()).unwrap();

            // Init Config
            info!("Init Config Store");
            init_config(app);

            // Register Global Shortcut
            match register_shortcut("all") {
                Ok(()) => {}
                Err(e) => error!("Failed to register global shortcut"),
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|_app_handle, event| {
            if let tauri::RunEvent::ExitRequested { api, .. } = event {
                api.prevent_exit();
            }
        });
}
