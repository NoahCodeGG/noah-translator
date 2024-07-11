use crate::APP;
use log::info;
use std::path::PathBuf;
use tauri::api::path;

pub fn get_app_cache_dir_path() -> PathBuf {
    let app_handle = APP.get().unwrap();
    let app_cache_dir_path = path::app_cache_dir(app_handle.config().as_ref()).unwrap();
    app_cache_dir_path
}

pub fn get_profiles_cache_dir_path() -> PathBuf {
    let app_cache_dir_path = get_app_cache_dir_path();
    let profiles_cache_dir_path = app_cache_dir_path.join("profiles");
    if !profiles_cache_dir_path.exists() {
        std::fs::create_dir_all(&profiles_cache_dir_path)
            .expect("Create Profiles Cache Dir Failed");
    }
    profiles_cache_dir_path
}

pub fn get_profile_cache_dir_path(profile_id: &str) -> PathBuf {
    let profiles_cache_dir_path = get_profiles_cache_dir_path();
    let profile_cache_dir_path = profiles_cache_dir_path.join(&profile_id);
    info!("profile_cache_dir_path: {:?}", profile_cache_dir_path);
    if !profile_cache_dir_path.exists() {
        std::fs::create_dir_all(&profile_cache_dir_path).expect("Create Profile Cache Dir Failed");
    }
    profile_cache_dir_path
}

pub fn get_profile_cache_file_path(profile_id: &str, file_name: &str) -> PathBuf {
    let profile_cache_dir_path = get_profile_cache_dir_path(profile_id);
    let profile_cache_file_path = profile_cache_dir_path.join(&file_name);
    if !profile_cache_file_path.exists() {
        std::fs::File::create(&profile_cache_file_path).expect("Create Profile Cache File Failed");
    }
    profile_cache_file_path
}
