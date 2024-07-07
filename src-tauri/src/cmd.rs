use crate::path::{
    get_cut_image_cache_path, get_profile_cache_dir_path, get_profile_translations_cache_dir_path,
    get_screenshot_cache_path,
};
use log::{error, info};
use std::path::PathBuf;
use xcap::image::GenericImage;
use xcap::{image, Monitor};

pub fn screenshot(monitor_name: &str, save_path: &str) {
    info!("Screenshot monitor: {}", monitor_name);
    let monitors = Monitor::all().unwrap();

    for monitor in monitors {
        if monitor.name() == monitor_name {
            let image = monitor.capture_image().unwrap();
            image.save(save_path).unwrap();
        }
    }
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
