use crate::api::sanitization;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TranslatedText {
    translated_text: Option<String>,
    error: Option<String>,
}

pub async fn translate_text(
    base_language: String,
    target_language: String,
    text: String,
) -> String {
    let send = HashMap::from([
        ("q", text.to_owned()),
        ("source", base_language.to_owned()),
        ("target", target_language.to_owned()),
        ("format", "text".to_owned()),
    ]);
    let req_client = reqwest::Client::new();
    let res = req_client
        .post("https://libretranslate.de/translate")
        .json(&send)
        .send()
        .await
        .expect("Couldn't get a response");
    //println!("Response Status: {:#?}", res.status());
    //println!("{:#?}", send);
    //println!("Response Body: {:#?}", res);

    let response = res
        .json::<TranslatedText>()
        .await
        .expect("Couldn't turn the response body into a proper struct");
    //println!("Response: {:#?}", response);

    /*match response.translated_text {
        Some(tt) => format!("({} to {}) {}", base_language, target_language, tt),

        None => "I couldn't translate the text".to_owned()
    };*/

    match response.error {
        Some(et) => format!(
            "🌲 I tried to reach out to the API but i couldn't, because i got \"{}\" as response",
            et
        ),

        None => sanitization::sanitize_text(format!(
            "({} > {}) {}",
            base_language,
            target_language,
            response.translated_text.unwrap()
        ))
        .await
        .unwrap(),
    }
}
