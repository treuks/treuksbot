use crate::api::sanitization;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TranslatedText {
    error: Option<String>,
    translation: Option<String>,
}

pub async fn translate_text(
    base_language: String,
    target_language: String,
    text: String,
) -> String {
    let req_client = reqwest::Client::new();
    let res = req_client
        .get(format!(
            "https://lingva.pussthecat.org/api/v1/{}/{}/{}",
            (&base_language),
            (&target_language),
            (&text)
        ))
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
            &base_language,
            &target_language,
            response.translation.unwrap()
        ))
        .await
        .unwrap(),
    }
}
