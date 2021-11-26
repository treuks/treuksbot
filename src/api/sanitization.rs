use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageResponse {
    banned: bool,
    input_message: String,
    banphrase_data: Option<BanPhraseData>                  
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BanPhraseData {
    id: i32,
    name: String,
    phrase: String,
    length: i32,
    permanent: bool,
    operator: String,
    case_sensitive: bool
}


pub async fn sanitize_text(text: String) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let mut message = HashMap::new();
    message.insert("message", text.to_owned());
    let req_client = reqwest::Client::new();
    let res = req_client.post("https://pajlada.pajbot.com/api/v1/banphrases/test")               
        .json(&message)
        .send()
        .await
        .expect("Couldn't get a response");
        //println!("{:#?}", message);
        //println!("Response Body: {:#?}", res);

    let js = res.json::<MessageResponse>().await.expect("Couldn't turn the response body into a proper struct");
    if js.banned == true {
        Ok("[REDACTED]".to_owned())
    } else {
        Ok(format!("🌲 {}", text))
    } 
        
}