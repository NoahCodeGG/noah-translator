use crate::cmd::screenshot_async;
use crate::monitor::get_current_monitor_xcap_id_by_tauri_monitor;
use crate::path::get_profile_cache_dir_path;
use crate::profile::update_profile_cache_config;
use crate::task::start_ocr_translate_task;
use crate::APP;
use log::{info, warn};
use nanoid::nanoid;
use serde_json::{from_str, json, Value};
use tauri::{LogicalPosition, LogicalSize, Manager, Window, WindowBuilder};

// Get daemon window instance
pub fn get_daemon_window() -> Window {
    let app_handle = APP.get().unwrap();
    match app_handle.get_window("daemon") {
        Some(v) => v,
        None => {
            warn!("Daemon window not found, create new daemon window!");
            WindowBuilder::new(
                app_handle,
                "daemon",
                tauri::WindowUrl::App("daemon.html".into()),
            )
            .title("Daemon")
            .visible(false)
            .build()
            .unwrap()
        }
    }
}

// Creating a window on the mouse monitor
fn build_window(label: &str, title: &str) -> (Window, bool) {
    let app_handle = APP.get().unwrap();
    match app_handle.get_window(label) {
        Some(v) => {
            info!("Window existence: {}", label);
            v.set_focus().unwrap();
            (v, true)
        }
        None => {
            info!("Window not existence, Creating new window: {}", label);
            let mut builder = WindowBuilder::new(
                app_handle,
                label,
                tauri::WindowUrl::App("index.html".into()),
            )
            .focused(true)
            .title(title)
            .visible(false)
            .decorations(false)
            .transparent(true);

            #[cfg(target_os = "macos")]
            {
                builder = builder
                    .title_bar_style(tauri::TitleBarStyle::Transparent)
                    .hidden_title(true);
            }
            let window = builder.build().unwrap();

            if label != "screenshot" {
                #[cfg(not(target_os = "linux"))]
                {
                    use window_shadows::set_shadow;
                    set_shadow(&window, true).unwrap_or_default();
                }
            }

            // #[cfg(debug_assertions)]
            // {
            //     window.open_devtools();
            // }

            (window, false)
        }
    }
}

pub fn quick_creation() {
    let screenshot_window = screenshot_window();
    let monitor = screenshot_window.current_monitor().unwrap().unwrap();
    let monitor_id = get_current_monitor_xcap_id_by_tauri_monitor(&monitor);
    let profile_id = nanoid!();
    let profile_cache_dir_path = get_profile_cache_dir_path(profile_id.clone());
    let screenshot_path_buf = profile_cache_dir_path.join("profile.png");
    let screenshot_path = screenshot_path_buf.to_str().unwrap();
    let screenshot_window_ = screenshot_window.clone();

    screenshot_async(monitor_id, screenshot_path);

    screenshot_window.listen("translate_area", move |event| {
        let data: Value = from_str(event.payload().unwrap()).unwrap();
        info!("Translate Area: {:?}", data);
        let x = data["x"].as_i64().unwrap() as u32;
        let y = data["y"].as_i64().unwrap() as u32;
        let width = data["width"].as_i64().unwrap() as u32;
        let height = data["height"].as_i64().unwrap() as u32;

        let dpi = monitor.scale_factor();
        let window_logical_size = LogicalSize::new(width, height);
        let window_size = window_logical_size.to_physical::<u32>(dpi);
        let window_logical_position = LogicalPosition::new(x, y);
        let window_position = window_logical_position.to_physical::<i32>(dpi);

        // 创建配置文件
        let config = json!({
            "translate_area": {
                "x": window_position.x,
                "y": window_position.y,
                "width": window_size.width,
                "height": window_size.height,
                "monitor_id": monitor_id
            }
        });
        update_profile_cache_config(profile_id.clone(), config.to_string());

        let translate_window = translate_window(&monitor, 0, 0, 0, 0);
        start_ocr_translate_task(&translate_window, profile_id.clone());
        screenshot_window_.unlisten(event.id())
    });
}

fn screenshot_window() -> Window {
    let (window, _exists) = build_window("screenshot", "Screenshot");
    let monitor = window.current_monitor().unwrap().unwrap();

    #[cfg(target_os = "macos")]
    {
        let monitor_size = monitor.size();
        let monitor_position = monitor.position();
        let dpi = monitor.scale_factor();
        let logical_size = monitor_size.to_logical::<f64>(dpi);
        let logical_position = monitor_position.to_logical::<f64>(dpi);
        window.set_decorations(false).unwrap();
        window.set_size(logical_size).unwrap();
        window.set_position(logical_position).unwrap();
    }

    #[cfg(not(target_os = "macos"))]
    {
        window.set_fullscreen(true).unwrap();
        window.set_skip_taskbar(true).unwrap();
    }

    window.set_resizable(false).unwrap();
    window.set_maximizable(false).unwrap();
    window.set_minimizable(false).unwrap();
    window.set_always_on_top(true).unwrap();
    window.set_focus().unwrap();
    window.show().unwrap();

    window
}

fn translate_window(monitor: &tauri::Monitor, x: i32, y: i32, width: u32, height: u32) -> Window {
    let (window, _exists) = build_window("translate", "Translate");
    if x != 0 && y != 0 && width != 0 && height != 0 {
        let dpi = monitor.scale_factor();
        let window_logical_size = LogicalSize::new(width, height);
        let window_size = window_logical_size.to_physical::<u32>(dpi);
        let window_logical_position = LogicalPosition::new(x, y);
        let window_position = window_logical_position.to_physical::<i32>(dpi);
        window.set_size(window_size).unwrap();
        window.set_position(window_position).unwrap();
    } else {
        window
            .set_min_size(Some(tauri::LogicalSize::new(100, 100)))
            .unwrap();
        window.set_size(tauri::LogicalSize::new(300, 200)).unwrap();

        #[cfg(target_os = "macos")]
        {
            let dpi = monitor.scale_factor();
            let monitor_position = monitor.position().to_logical::<i32>(dpi);
            let monitor_size = monitor.size().to_logical::<i32>(dpi);
            let window_position = LogicalPosition::new(
                monitor_position.x + (monitor_size.width - 300) / 2,
                monitor_position.y + (monitor_size.height - 200) / 2,
            );
            window.set_position(window_position).unwrap();
        }

        #[cfg(not(target_os = "macos"))]
        {
            window.center().unwrap();
        }
    }

    window.set_maximizable(false).unwrap();
    window.set_minimizable(false).unwrap();
    window.set_always_on_top(true).unwrap();
    window.set_focus().unwrap();
    window.show().unwrap();

    window
}
