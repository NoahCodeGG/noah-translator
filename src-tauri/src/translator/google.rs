use async_trait::async_trait;
use serde_json::Value;
use tauri::Url;

use crate::config;

use super::translator::Translator;

pub struct GoogleTranslator {
    target_lang: String,
}

impl GoogleTranslator {
    pub fn new() -> Self {
        GoogleTranslator {
            target_lang: config::get("target_lang")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string(),
        }
    }
}

#[async_trait]
impl Translator for GoogleTranslator {
    async fn translate(&self, text: &str, source_lang: &str) -> Result<String, String> {
        const GOOGLE_TRANSLATE_API_URL: &str = "https://translate.googleapis.com/translate_a/single?dt=at&dt=bd&dt=ex&dt=ld&dt=md&dt=qca&dt=rw&dt=rm&dt=ss&dt=t";
        let params = [
            ("client", "gtx"),
            ("sl", source_lang),
            ("tl", self.target_lang.as_str()),
            ("hl", self.target_lang.as_str()),
            ("ie", "UTF-8"),
            ("oe", "UTF-8"),
            ("otf", "1"),
            ("ssel", "0"),
            ("tsel", "0"),
            ("kc", "7"),
            ("q", text),
        ];
        let url = Url::parse_with_params(GOOGLE_TRANSLATE_API_URL, &params).unwrap();
        let response = reqwest::get(url).await.unwrap();
        if response.status().is_success() {
            let response: Value = response.json().await.unwrap();
            if response.is_array() {
                let mut result = String::new();
                for item in response.as_array().unwrap() {
                    if item.is_array() {
                        for sub_item in item.as_array().unwrap() {
                            if sub_item.is_string() {
                                result.push_str(sub_item.as_str().unwrap());
                            }
                        }
                    }
                }

                return Ok(result.trim().to_string());
            } else {
                return Err("Failed to translate text!".to_string());
            }
        } else {
            Err("Failed to translate text!".to_string())
        }
    }
}
