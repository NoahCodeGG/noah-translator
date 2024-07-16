use log::{error, info};
use serde_json::{json, Value};

pub async fn translate(text: &str, from: &str, to: &str) -> Result<String, String> {
    let client = reqwest::Client::new();
    const TOKEN_URL: &str = "https://edge.microsoft.com/translate/auth";
    let token_rsp = client
        .get(TOKEN_URL)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/113.0.0.0 Safari/537.36 Edg/113.0.1774.42")
        .send()
        .await
        .unwrap();

    if token_rsp.status().is_success() {
        const TRANSLATE_URL: &str = "https://api-edge.cognitive.microsofttranslator.com/translate";
        let token = token_rsp.text().await.unwrap();
        info!("token: {:?}", token);

        let translate_query = vec![
            ("from", ""),
            ("to", "zh-Hans"),
            ("api-version", "3.0"),
            ("includeSentenceLength", "true"),
        ];

        let translate_json = json!([{"Text": text}]);

        let translate_rsp = client
            .post(TRANSLATE_URL)
            .header("accept", "*/*")
            .header("accept-language", "zh-TW,zh;q=0.9,ja;q=0.8,zh-CN;q=0.7,en-US;q=0.6,en;q=0.5")
            .header("authorization", format!("Bearer {}", token))
            .header("cache-control", "no-cache")
            .header("content-type", "application/json")
            .header("pragma", "no-cache")
            .header("sec-ch-ua","\"Microsoft Edge\";v=\"113\", \"Chromium\";v=\"113\", \"Not-A.Brand\";v=\"24\"")
            .header("sec-ch-ua", "?0")
            .header("sec-ch-ua-platform", "\"Windows\"")
            .header("sec-fetch-dest", "empty")
            .header("sec-fetch-mode", "cors")
            .header("sec-fetch-site", "cross-site")
            .header("Referer", "https://appsumo.com/")
            .header("Referrer-Policy", "strict-origin-when-cross-origin")
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/113.0.0.0 Safari/537.36 Edg/113.0.1774.42")
            .query(&translate_query)
            .json(&translate_json)
            .send()
            .await
            .unwrap();

        if translate_rsp.status().is_success() {
            let result = translate_rsp.text().await.unwrap();
            let result: Value = serde_json::from_str(&result).unwrap();
            if result[0]["translations"].is_array() {
                return Ok(result[0]["translations"][0]["text"]
                    .as_str()
                    .unwrap()
                    .trim()
                    .to_string());
            } else {
                error!(
                    "Translate translations is not array: {:?}",
                    result.to_string()
                );
                return Err(result.to_string());
            }
        } else {
            error!(
                "Translate Request Failed status: {:?}",
                translate_rsp.status()
            );
            return Err("Translate Request Failed".to_string());
        }
    } else {
        error!("Get Token Failed status: {:?}", token_rsp.status());
        return Err("Get Token Failed".to_string());
    }
}

// pub async fn translate(text: &str, from: &str, to: &str) -> Result<String, reqwest::Error> {
//     const GOOGLE_TRANSLATE_API_URL: &str = "https://translate.googleapis.com/translate_a/single?dt=at&dt=bd&dt=ex&dt=ld&dt=md&dt=qca&dt=rw&dt=rm&dt=ss&dt=t";
//     let params = [
//         ("client", "gtx"),
//         ("sl", from),
//         ("tl", to),
//         ("hl", to),
//         ("ie", "UTF-8"),
//         ("oe", "UTF-8"),
//         ("otf", "1"),
//         ("ssel", "0"),
//         ("tsel", "0"),
//         ("kc", "7"),
//         ("q", text),
//     ];
//     let url = Url::parse_with_params(GOOGLE_TRANSLATE_API_URL, &params).unwrap();
//     let response = get(url).await?.text().await?;
//     let response: Value = serde_json::from_str(&response).unwrap();
//     // info!("google translate response: {:?}", response);
//     let mut result = String::new();
//     for sentence in response[0].as_array().unwrap() {
//         if sentence[0].is_string() {
//             result.push_str(&sentence[0].as_str().unwrap());
//         }
//     }
//     Ok(result.trim().to_string())
// }
