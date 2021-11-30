use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub async fn is_up(text: String) {
    let req_client = reqwest::Client::new();
    let res = req_client
        .head(text)
        .send()
        .await
        .expect("Couldn't get a response");
    //println!("{:#?}", text);
    println!("Response Body: {:#?}", res);
}
