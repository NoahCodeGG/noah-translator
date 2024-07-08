use log::error;
use std::path::PathBuf;
use xcap::image;
use xcap::image::GenericImage;

pub fn screenshot(monitor_id: u32, save_path: &str) {
    let monitors = xcap::Monitor::all().unwrap();

    for monitor in monitors {
        if monitor.id() == monitor_id {
            let image = monitor.capture_image().unwrap();
            image.save(save_path).unwrap();
        }
    }
}

pub fn screenshot_async(monitor_id: u32, save_path: &str) {
    let path = PathBuf::from(save_path);
    
    tauri::async_runtime::spawn(async move {
        screenshot(monitor_id, path.to_str().unwrap());
    });
}

pub fn cut_image(image_path: &str, save_path: &str, x: u32, y: u32, width: u32, height: u32) {
    let image_path = PathBuf::from(&image_path);
    if !image_path.exists() {
        return;
    }

    let mut img = match image::open(&image_path) {
        Ok(v) => v,
        Err(e) => {
            error!("{:?}", e.to_string());
            return;
        }
    };

    let sub_img = img.sub_image(x, y, width, height);
    return match sub_img.to_image().save(&save_path) {
        Ok(_) => {}
        Err(e) => {
            error!("{:?}", e.to_string());
        }
    };
}
