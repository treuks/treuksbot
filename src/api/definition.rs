use serde::{Deserialize, Serialize};
use reqwest::{Client, StatusCode};
use std::collections::HashMap;
//use hyper::header::{Headers, UserAgent}; // *TODO: Use this when implementing wikipedia

// *TODO: Implement urban dictionary, and wikipedia api

pub async fn translate_text(word: String) /* -> String */{
    let req_client = reqwest::Client::new();
    let res = req_client.get(format!("https://api.dictionaryapi.dev/api/v2/entries/en/{}", word));
    println!("Response Body: {:#?}", res);

    //let response = res.json::<reee>().await.expect("Couldn't turn the response body into a proper struct");

} 

