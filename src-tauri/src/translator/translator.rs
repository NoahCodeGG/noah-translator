use async_trait::async_trait;

#[async_trait]
pub trait Translator {
    async fn translate(&self, text: &str, source_lang: &str) -> Result<String, String>;
}
