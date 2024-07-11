use crate::path::get_profile_cache_dir_path;

pub fn update_profile_cache_config(profile_id: &str, contents: &str) {
    let profile_cache_dir_path = get_profile_cache_dir_path(profile_id);
    let profile_cache_config_path = profile_cache_dir_path.join("config.json");
    std::fs::write(profile_cache_config_path, contents).unwrap();
}

pub fn get_profile_cache_config(profile_id: &str) -> String {
    let profile_cache_dir_path = get_profile_cache_dir_path(profile_id);
    let profile_cache_config_path = profile_cache_dir_path.join("config.json");
    std::fs::read_to_string(profile_cache_config_path).unwrap()
}
