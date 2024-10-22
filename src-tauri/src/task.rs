use crate::cmd::{cut_image, screenshot};
use crate::path::get_profile_cache_dir_path;
use crate::profile::{get_profile_cache_config, update_profile_cache_config};
use crate::system_ocr::system_ocr;
use crate::translate::translate;
use crate::APP;
use log::info;
use tauri::{Window, WindowEvent};
use tokio::sync::mpsc;

pub fn start_ocr_translate_task(window: &Window, profile_id: &str) {
    let (ocr_translate_sender, mut ocr_translate_receiver) = mpsc::channel::<String>(1);
    let profile_id_str = profile_id.to_owned();
    info!("Start OCR Translate Task: {:?}", profile_id_str);
    let config_str = get_profile_cache_config(&profile_id_str);
    let config: serde_json::Value = serde_json::from_str(&config_str).unwrap();
    let app_handle = APP.get().unwrap();

    window.on_window_event(move |event| {
        if let WindowEvent::Moved(position) = event {
            let config_str = get_profile_cache_config(profile_id_str.as_str());
            let mut config: serde_json::Value = serde_json::from_str(&config_str).unwrap();
            config["translate_area"]["x"] = position.x.into();
            config["translate_area"]["y"] = position.y.into();
            update_profile_cache_config(profile_id_str.as_str(), config.to_string().as_str());
        }
    });

    let profile_id_str_clone = profile_id.to_owned().clone();
    let ocr_translate_handle = tauri::async_runtime::spawn(async move {
        let translate_area_str = config["translate_area"].to_string();
        let translate_area: serde_json::Value = serde_json::from_str(&translate_area_str).unwrap();
        info!("Config Translate Area: {:?}", translate_area);
        let x = translate_area["x"].as_i64().unwrap();
        let y = translate_area["y"].as_i64().unwrap();
        let width = translate_area["width"].as_i64().unwrap();
        let height = translate_area["height"].as_i64().unwrap();
        let monitor_id = translate_area["monitor_id"].as_u64().unwrap() as u32;

        loop {
            let profile_cache_dir_path = get_profile_cache_dir_path(&profile_id_str_clone);
            info!("Profile Cache Dir Path: {:?}", profile_cache_dir_path);
            let screenshot_path = {
                let path = profile_cache_dir_path.join("noah_translate.png");
                path.to_string_lossy().into_owned()
            };
            info!("Screenshot Path: {:?}", screenshot_path);

            screenshot(monitor_id, screenshot_path.as_str());
            info!("Screenshot: {:?}", screenshot_path);

            let cut_image_path = {
                let path = profile_cache_dir_path.join("noah_translate_cut.png");
                path.to_string_lossy().into_owned()
            };

            cut_image(
                screenshot_path.as_str(),
                cut_image_path.as_str(),
                x as u32,
                y as u32,
                width as u32,
                height as u32,
            );
            info!("Cut Image: {:?}", cut_image_path);

            let ocr_text = system_ocr(app_handle, cut_image_path.as_str(), "auto").unwrap();
            info!("OCR Text: {:?}", ocr_text);
            // TODO config source language
            let result = translate(&ocr_text, "auto", "zh-CN").await.unwrap();
            info!("Translate Result: {:?}", result);
            ocr_translate_sender.send(result).await.unwrap();
            // TODO config interval
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    });

    let ocr_translate_result_emit_handle_window = window.clone();
    let ocr_translate_result_emit_handle = tauri::async_runtime::spawn(async move {
        while let Some(result) = ocr_translate_receiver.recv().await {
            ocr_translate_result_emit_handle_window
                .emit("realtime_translation", Some(result))
                .unwrap();
        }
    });

    let window_ = window.clone();
    window.listen("close", move |event| {
        ocr_translate_handle.abort();
        ocr_translate_result_emit_handle.abort();
        window_.unlisten(event.id())
    });
}
