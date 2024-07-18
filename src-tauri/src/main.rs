// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::config::init_config;
use crate::shortcut::register_shortcut;
use log::{error, info};
use once_cell::sync::OnceCell;
use tauri::api::notification::Notification;
use tauri::Manager;
use tauri_plugin_log::LogTarget;
use tray::{tray_event_handler, update_tray};
mod cmd;
mod config;
mod monitor;
mod path;
mod profile;
mod shortcut;
mod system_ocr;
mod task;
mod translator;
mod tray;
mod window;

pub static APP: OnceCell<tauri::AppHandle> = OnceCell::new();

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _, cwd| {
            Notification::new(&app.config().tauri.bundle.identifier)
                .title("The program is already running. Please do not start it again!")
                .body(cwd)
                .icon("pot")
                .show()
                .unwrap();
        }))
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets([LogTarget::LogDir, LogTarget::Stdout, LogTarget::Webview])
                .build(),
        )
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            Some(vec![]),
        ))
        .system_tray(tauri::SystemTray::new())
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

            // Update Tray Menu
            update_tray(app.app_handle());

            // Register Global Shortcut
            match register_shortcut("all") {
                Ok(()) => {}
                Err(e) => error!("Failed to register global shortcut"),
            }

            Ok(())
        })
        .on_system_tray_event(tray_event_handler)
        .invoke_handler(tauri::generate_handler![])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|_app_handle, event| {
            if let tauri::RunEvent::ExitRequested { api, .. } = event {
                api.prevent_exit();
            }
        });
}
