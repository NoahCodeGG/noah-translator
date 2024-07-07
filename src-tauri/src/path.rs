use std::path::PathBuf;
use tauri::api::path;
use crate::APP;

pub fn get_app_cache_dir_path() -> PathBuf {
    let app_handle = APP.get().unwrap();
    let app_cache_dir_path = path::app_cache_dir(app_handle.config().as_ref()).unwrap();
    app_cache_dir_path
}

pub fn get_profiles_cache_dir_path() -> PathBuf {
    let app_cache_dir_path = get_app_cache_dir_path();
    let profiles_cache_dir_path = app_cache_dir_path.join("profiles");
    if !profiles_cache_dir_path.exists() {
        std::fs::create_dir_all(&profiles_cache_dir_path).expect("Create Profiles Cache Dir Failed");
    }
    profiles_cache_dir_path
}

pub fn get_profile_cache_dir_path(profile_id: &str) -> PathBuf {
    let profiles_cache_dir_path = get_profiles_cache_dir_path();
    let profile_cache_dir_path = profiles_cache_dir_path.join(profile_id);
    if !profile_cache_dir_path.exists() {
        std::fs::create_dir_all(&profile_cache_dir_path).expect("Create Profile Cache Dir Failed");
    }
    profile_cache_dir_path
}

pub fn get_profile_cache_file_path(profile_id: &str, file_name: &str) -> PathBuf {
    let profile_cache_dir_path = get_profile_cache_dir_path(profile_id);
    let profile_cache_file_path = profile_cache_dir_path.join(file_name);
    profile_cache_file_path
}

pub fn get_profile_translations_cache_dir_path(profile_id: &str) -> PathBuf {
    let profile_cache_dir_path = get_profile_cache_dir_path(profile_id);
    let profile_translations_cache_dir_path = profile_cache_dir_path.join("translations");
    if !profile_translations_cache_dir_path.exists() {
        std::fs::create_dir_all(&profile_translations_cache_dir_path).expect("Create Profile Translations Cache Dir Failed");
    }
    profile_translations_cache_dir_path
}

pub fn get_screenshot_cache_path(screenshot_type: &str, profile_id: &String) -> String {
    let screenshot_cache_dir_path;
    let screenshot_name;

    match screenshot_type {
        "profile" => {
            screenshot_cache_dir_path = get_profile_cache_dir_path(profile_id);
            screenshot_name = String::from("profile.png");
        }
        "translate" => {
            let now = chrono::Local::now();
            screenshot_cache_dir_path = get_profile_translations_cache_dir_path(profile_id);
            screenshot_name = now.format("%Y-%m-%d %H:%M:%S.png").to_string();
        }
        _ => panic!("Invalid screenshot type: {}", screenshot_type),
    }

    screenshot_cache_dir_path
        .join(screenshot_name)
        .to_str()
        .unwrap()
        .to_string()
}

pub fn get_cut_image_cache_path(cut_image_type: &str, profile_id: &String) -> String {
    let screenshot_cache_dir_path;
    let screenshot_name;

    match cut_image_type {
        "translate" => {
            let now = chrono::Local::now();
            screenshot_cache_dir_path = get_profile_translations_cache_dir_path(profile_id);
            screenshot_name = now.format("%Y-%m-%d %H:%M:%S_cut.png").to_string();
        }
        _ => panic!("Invalid cut image type: {}", cut_image_type),
    }

    screenshot_cache_dir_path
        .join(screenshot_name)
        .to_str()
        .unwrap()
        .to_string()
}
