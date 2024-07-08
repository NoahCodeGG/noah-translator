use crate::cmd::{cut_image, screenshot};
use crate::path::{get_profile_cache_file_path, get_profile_translations_cache_dir_path};
use crate::system_ocr::system_ocr;
use crate::translate::translate;
use crate::APP;
use log::info;
use tauri::Window;
use tokio::sync::mpsc;

pub fn start_ocr_translate_task(window: &Window, profile_id: &String) {
    let (ocr_translate_sender, mut ocr_translate_receiver) = mpsc::channel::<String>(1);
    let config_cache_path = get_profile_cache_file_path(profile_id, "config.json");
    let config: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&config_cache_path).unwrap()).unwrap();
    let profile_translations_cache_dir_path = get_profile_translations_cache_dir_path(profile_id);
    let app_handle = APP.get().unwrap();
    
    let ocr_translate_handle = tauri::async_runtime::spawn(async move {
        let translate_area_str = config["translate_area"].to_string();
        let translate_area: serde_json::Value = serde_json::from_str(&translate_area_str).unwrap();
        info!("Translate Area: {:?}", translate_area);
        let x = translate_area["x"].as_i64().unwrap();
        let y = translate_area["y"].as_i64().unwrap();
        let width = translate_area["width"].as_i64().unwrap();
        let height = translate_area["height"].as_i64().unwrap();
        let monitor_id = translate_area["monitor_id"].as_u64().unwrap() as u32;

        loop {
            let now = chrono::Local::now();
            let screenshot_name = now.format("%Y-%m-%d %H:%M:%S.png").to_string();
            let screenshot_path_buf = profile_translations_cache_dir_path.join(&screenshot_name);
            let screenshot_path = screenshot_path_buf.to_str().unwrap();
            screenshot(monitor_id, &screenshot_path);

            let cut_image_name = now.format("%Y-%m-%d %H:%M:%S_cut.png").to_string();
            let cut_image_path_buf = profile_translations_cache_dir_path.join(&cut_image_name);
            let cut_image_path = cut_image_path_buf.to_str().unwrap();
            cut_image(
                screenshot_path,
                cut_image_path,
                x as u32,
                y as u32,
                width as u32,
                height as u32,
            );

            let ocr_text = system_ocr(app_handle, cut_image_path, "auto").unwrap();
            // TODO config source language
            let result = translate(&ocr_text, "auto", "zh-CN").await.unwrap();
            ocr_translate_sender.send(result).await.unwrap();
            // TODO config interval
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    });
    
    let ocr_translate_result_emit_handle_window = window.clone();
    let ocr_translate_result_emit_handle = tauri::async_runtime::spawn(async move {
        while let Some(result) = ocr_translate_receiver.recv().await {
            ocr_translate_result_emit_handle_window.emit("realtime_translation", Some(result)).unwrap();
        }
    });

    let window_ = window.clone();
    window.listen("close", move |event| {
        ocr_translate_handle.abort();
        ocr_translate_result_emit_handle.abort();
        window_.unlisten(event.id())
    });
}
