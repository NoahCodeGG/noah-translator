use std::{collections::HashMap, sync::Mutex};

use tauri::Manager;

use crate::{
    config::get_config,
    translator::{bing::BingTranslator, google::GoogleTranslator, translator::Translator},
    APP,
};

pub struct TranslatorPluginLoaderWrapper(pub Mutex<TranslatorPluginLoader>);

dyn_clone::clone_trait_object!(Translator);

#[derive(Clone)]
pub struct TranslatorPluginLoader {
    plugins: HashMap<String, Box<dyn Translator + Send>>,
}

impl TranslatorPluginLoader {
    pub fn new() -> Self {
        let mut plugins: HashMap<String, Box<dyn Translator + Send>> = HashMap::new();
        plugins.insert("google".to_string(), Box::new(GoogleTranslator::new()));
        plugins.insert("bing".to_string(), Box::new(BingTranslator::new()));

        TranslatorPluginLoader { plugins }
    }

    pub fn get_translator(&self, name: &str) -> Option<&Box<dyn Translator + Send>> {
        self.plugins.get(name)
    }
}

pub fn init_plugin(app: &mut tauri::App) {
    let translator_plugin_loader = TranslatorPluginLoader::new();
    app.manage(TranslatorPluginLoaderWrapper(Mutex::new(
        translator_plugin_loader,
    )));
}

pub fn get_translator() -> Option<Box<dyn Translator + Send>> {
    let state = APP.get().unwrap().state::<TranslatorPluginLoaderWrapper>();
    let loader = state.0.lock().unwrap();
    let translate_plugin_name = get_config("translate_plugin_name").unwrap();
    loader
        .get_translator(translate_plugin_name.as_str().unwrap())
        .map(|t| t.clone())
}
