use async_trait::async_trait;
use dyn_clone::DynClone;

#[async_trait]
pub trait Translator: DynClone {
    async fn translate(&self, text: &str, source_lang: &str) -> Result<String, String>;
}
