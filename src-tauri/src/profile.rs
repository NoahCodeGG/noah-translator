use crate::path::get_profile_cache_dir_path;
use serde_json::json;

pub fn create_profile_cache_config(
    profile_id: &str,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    monitor_name: &String,
) {
    let profile_cache_dir_path = get_profile_cache_dir_path(profile_id);
    let profile_cache_config_path = profile_cache_dir_path.join("config.json");
    let config = json!({
            "translate_area": {
            "x": x,
            "y": y,
            "width": width,
            "height": height,
            "monitor_name": monitor_name
        }
    });
    std::fs::write(
        profile_cache_config_path,
        serde_json::to_string_pretty(&config).unwrap(),
    )
    .unwrap();
}
