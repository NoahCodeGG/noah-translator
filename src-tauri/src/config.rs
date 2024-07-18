use crate::APP;
use log::{info, warn};
use serde_json::{json, Value};
use std::sync::Mutex;
use tauri::api::os::locale;
use tauri::api::path::app_config_dir;
use tauri::{Manager, Wry};
use tauri_plugin_store::{Store, StoreBuilder};

pub struct StoreWrapper(pub Mutex<Store<Wry>>);

pub fn init_config(app: &mut tauri::App) {
    let config_path = app_config_dir(app.config().as_ref())
        .unwrap()
        .join("config.json");
    info!("Load config from: {:?}", config_path);
    let mut store = StoreBuilder::new(app.handle(), config_path).build();

    match store.load() {
        Ok(_) => info!("Config loaded"),
        Err(e) => {
            warn!("Config load error: {:?}", e);
            info!("Config not found, creating new config");
        }
    }
    app.manage(StoreWrapper(Mutex::new(store)));

    // Check default config
    check_default_config();
}

pub fn get_config(key: &str) -> Option<Value> {
    let state = APP.get().unwrap().state::<StoreWrapper>();
    let store = state.0.lock().unwrap();
    match store.get(key) {
        Some(value) => Some(value.clone()),
        None => None,
    }
}

pub fn set_config<T: serde::ser::Serialize>(key: &str, value: T) {
    let state = APP.get().unwrap().state::<StoreWrapper>();
    let mut store = state.0.lock().unwrap();
    store.insert(key.to_string(), json!(value)).unwrap();
    store.save().unwrap();
}

fn check_config_with_default_value<T: serde::ser::Serialize>(key: &str, default: T) {
    match get_config(key) {
        Some(..) => {}
        None => set_config(key, default),
    }
}

pub fn check_default_config() {
    let local = locale().unwrap();
    check_config_with_default_value("auto_start", false);
    check_config_with_default_value("check_update", true);
    check_config_with_default_value("app_language", local.clone());
    check_config_with_default_value("app_theme", "system");
    check_config_with_default_value("translate_plugin_name", "bing");
    check_config_with_default_value("translate_target_language", local.clone());
    check_config_with_default_value("ocr_interval", 1000);
}
