use log::{info, warn};

use crate::window::get_daemon_window;

pub fn get_current_monitor_by_mouse_position(x: i32, y: i32) -> tauri::Monitor {
    info!("Mouse position: {}, {}", x, y);
    let daemon_window = get_daemon_window();
    let monitors = daemon_window.available_monitors().unwrap();

    for monitor in monitors {
        let size = monitor.size();
        let position = monitor.position();

        if x >= position.x
            && x <= (position.x + size.width as i32)
            && y >= position.y
            && y <= (position.y + size.height as i32)
        {
            info!("Current Monitor: {:?}", monitor);
            return monitor;
        }
    }
    warn!("Current Monitor not found, using primary monitor");
    daemon_window.primary_monitor().unwrap().unwrap()
}

pub fn get_tauri_monitor_by_xcap_monitor_id(xcap_monitor_id: u32) -> tauri::Monitor {
    let xcap_monitors = xcap::Monitor::all().unwrap();
    for xcap_monitor in xcap_monitors {
        if xcap_monitor.id() == xcap_monitor_id {
            let daemon_window = get_daemon_window();
            let tauri_monitors = daemon_window.available_monitors().unwrap();
            for tauri_monitor in tauri_monitors {
                if tauri_monitor.position().x == xcap_monitor.x()
                    && tauri_monitor.position().y == xcap_monitor.y()
                {
                    return tauri_monitor;
                }
            }
        }
    }

    panic!("No monitor found with monitor_id: {}", xcap_monitor_id);
}

pub fn get_current_monitor_xcap_id_by_tauri_monitor(tauri_monitor: &tauri::Monitor) -> u32 {
    let xcap_monitors = xcap::Monitor::all().unwrap();
    for xcap_monitor in xcap_monitors {
        if xcap_monitor.x() == tauri_monitor.position().x
            && xcap_monitor.y() == tauri_monitor.position().y
        {
            return xcap_monitor.id();
        }
    }

    panic!("No monitor found with tauri monitor position: {:?}", tauri_monitor.position());
}
