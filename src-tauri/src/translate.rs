use reqwest::{get, Url};
use serde_json::Value;

pub async fn translate(text: &str, from: &str, to: &str) -> Result<String, reqwest::Error> {
    const GOOGLE_TRANSLATE_API_URL: &str = "https://translate.googleapis.com/translate_a/single?dt=at&dt=bd&dt=ex&dt=ld&dt=md&dt=qca&dt=rw&dt=rm&dt=ss&dt=t";
    let params = [
        ("client", "gtx"),
        ("sl", from),
        ("tl", to),
        ("hl", to),
        ("ie", "UTF-8"),
        ("oe", "UTF-8"),
        ("otf", "1"),
        ("ssel", "0"),
        ("tsel", "0"),
        ("kc", "7"),
        ("q", text),
    ];
    let url = Url::parse_with_params(GOOGLE_TRANSLATE_API_URL, &params).unwrap();
    let response = get(url).await?.text().await?;
    let response: Value = serde_json::from_str(&response).unwrap();
    // info!("google translate response: {:?}", response);
    let mut result = String::new();
    for sentence in response[0].as_array().unwrap() {
        if sentence[0].is_string() {
            result.push_str(&sentence[0].as_str().unwrap());
        }
    }
    Ok(result.trim().to_string())
}
