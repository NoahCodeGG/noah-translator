use log::info;
use tauri::CustomMenuItem;
use tauri::Manager;
use tauri::SystemTrayEvent;
use tauri::SystemTrayMenu;
use tauri::SystemTrayMenuItem;
use tauri::{AppHandle, GlobalShortcutManager};

use crate::window::config_window;
use crate::window::quick_creation;

#[tauri::command]
pub fn update_tray(app_handle: tauri::AppHandle) {
    let tray_handle = app_handle.tray_handle();
    tray_handle.set_menu(tray_menu()).unwrap();
}

pub fn tray_event_handler<'a>(app: &'a AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "quick_creation" => on_quick_creation_click(),
            "config" => on_config_click(),
            "view_log" => on_view_log_click(app),
            "restart" => on_restart_click(app),
            "quit" => on_quit_click(app),
            _ => {}
        },
        _ => {}
    }
}

fn on_quick_creation_click() {
    quick_creation();
}

fn on_config_click() {
    config_window();
}

fn on_view_log_click(app: &AppHandle) {
    use tauri::api::path::app_log_dir;
    let log_path = app_log_dir(&app.config()).unwrap();
    tauri::api::shell::open(&app.shell_scope(), log_path.to_str().unwrap(), None).unwrap();
}

fn on_restart_click(app: &AppHandle) {
    info!("============== Restart App ==============");
    app.restart();
}

fn on_quit_click(app: &AppHandle) {
    app.global_shortcut_manager().unregister_all().unwrap();
    info!("============== Quit App ==============");
    app.exit(0);
}

fn tray_menu() -> tauri::SystemTrayMenu {
    let quick_creation = CustomMenuItem::new("quick_creation", "快速创建").accelerator("f7");
    let view_log = CustomMenuItem::new("view_log", "查看日志");
    let config = CustomMenuItem::new("config", "偏好设置");
    let restart = CustomMenuItem::new("restart", "重启应用");
    let quit = CustomMenuItem::new("quit", "退出");

    SystemTrayMenu::new()
        .add_item(quick_creation)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(config)
        .add_item(view_log)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(restart)
        .add_item(quit)
}
