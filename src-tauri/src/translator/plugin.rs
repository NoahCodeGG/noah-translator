use std::collections::HashMap;

use super::{bing::BingTranslator, google::GoogleTranslator, translator::Translator};

pub struct TranslatorPluginLoader {
    plugins: HashMap<String, Box<dyn Translator>>,
}

impl TranslatorPluginLoader {
    pub fn new() -> Self {
        let mut plugins: HashMap<String, Box<dyn Translator>> = HashMap::new();
        plugins.insert("google".to_string(), Box::new(GoogleTranslator::new()));
        plugins.insert("bing".to_string(), Box::new(BingTranslator::new()));

        TranslatorPluginLoader { plugins }
    }

    pub fn get_translator(&self, name: &str) -> Option<&Box<dyn Translator>> {
        self.plugins.get(name)
    }
}
