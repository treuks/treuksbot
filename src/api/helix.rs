use serde::{Deserialize, Serialize};
use reqwest::Client;
use reqwest::StatusCode;

use crate::api::types;
use std::sync::Arc;
use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};
use toml_edit::{Document, value};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct UserInformation {
    error: Option<String>,
    status: Option<i32>,
    message: Option<String>,
    data: Option<Vec<UserData>>
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct UserData {
    id: String,
    login: String,
    display_name: Option<String>,
    r#type: Option<String>,
    broadcaster_type: String,
    description: Option<String>,
    profile_image_url: String,
    offline_image_url: String,
    view_count: Option<i32>,
    email: Option<String>,
    created_at: String
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct ChannelInformation {
    error: Option<String>,
    status: Option<i32>,
    message: Option<String>,
    data: Option<Vec<Data>>
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Data {
    broadcaster_id: String,
    broadcaster_login: String,
    broadcaster_name: String,
    broadcaster_language: String,
    game_id: String,
    game_name: String,
    title: String,
    delay: i32
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct OauthResponse {
    error: Option<String>,
    status: Option<i32>,
    message: Option<String>,
    access_token: String,
    refresh_token: Option<String>,
    expires_in: i32,
    scope: Vec<String>,
    token_type: String,
}

pub async fn check_oauth_token(credentials: Arc::<types::Config>, credentials_location: String){

    let f = File::create(&credentials_location).unwrap();
    let mut f = BufWriter::new(f);

    let scopes = "analytics:read:extensions user:edit user:read:email clips:edit bits:read analytics:read:games user:edit:broadcast user:read:broadcast chat:read chat:edit channel:moderate channel:read:subscriptions whispers:read whispers:edit moderation:read channel:read:redemptions channel:edit:commercial channel:read:hype_train channel:manage:extensions channel:manage:broadcast user:edit:follows channel:manage:redemptions channel:read:editors channel:manage:videos user:read:blocked_users user:manage:blocked_users user:read:subscriptions user:read:follows channel:manage:polls channel:manage:predictions channel:read:polls channel:read:predictions moderator:manage:automod channel:manage:schedule channel:read:goals";
    eprintln!("Verifying if the Oauth token is okay");
    let req_client = reqwest::Client::new();
    let res = req_client.post("https://id.twitch.tv/oauth2/token")
        .query(&[("client_id", &credentials.secret.client_id), 
        ("client_secret", &credentials.secret.client_secret), 
        ("grant_type", &"client_credentials".to_owned()),
        ("scope", &scopes.to_owned())
        ])
        .send()
        .await
        .expect("Couldn't get a response");
        //let response_status = res.status();
        //println!("Response Status: {:#?}", res.status());
        //println!("{:#?}", res);
        
    let response = res.json::<OauthResponse>().await.expect("You probably put wrong data in the config"); 

    let file = fs::read_to_string(credentials_location).expect("I couldn't find the file");
    let mut doc = file.parse::<Document>().expect("Invalid doc");
    
    println!("Response Body: {:#?}", response);
    if response.expires_in <= 60 {
        //doc["oauth"] = toml_edit::(&response.access_token);
        f.write(doc.to_string().as_bytes()).unwrap();
        eprintln!("Refreshed the Oauth token");
    } else {
        doc["oauth"] = toml_edit::value(&response.access_token);
        f.write(doc.to_string().as_bytes()).unwrap();
        eprintln!("Everything seems okay");
    }
        
}

pub async fn get_broadcaster_id(credentials: Arc::<types::Config>) -> Result<std::string::String, Box<(dyn std::error::Error + Send + Sync + 'static)>> {

    let req_client = reqwest::Client::new();
    let res = req_client.get("https://api.twitch.tv/helix/users")
        .bearer_auth(credentials.secret.oauth.to_owned())
        .header("Client-Id", credentials.secret.client_id.to_owned())
        .query(&[("login", credentials.secret.channel_name.to_owned())])
        .send()
        .await
        .expect("Couldn't get a response");
        //let response_status = res.status();
        //println!("Response Status: {:#?}", res.status());
        //println!("{:#?}", send);
        

    let response = res.json::<UserInformation>().await.expect("Couldn't turn the response body into a proper struct"); 
    //println!("Response Body: {:#?}", response);
    match response.data {
        Some(so) => Ok(so[0].id.to_owned()),

        None => Ok("Bruh moment".to_owned())

    }
        
} 


pub async fn get_title(credentials: Arc::<types::Config>) -> Result<std::string::String, Box<(dyn std::error::Error + Send + Sync + 'static)>> {

    let req_client = reqwest::Client::new();
    let res = req_client.get("https://api.twitch.tv/helix/channels")
        .bearer_auth(credentials.secret.oauth.to_owned())
        .header("Client-Id", credentials.secret.client_id.to_owned())
        .query(&[("broadcaster_id", get_broadcaster_id(credentials).await.unwrap())])
        .send()
        .await
        .expect("Couldn't get a response");
        //let response_status = res.status();
        //println!("Response Status: {:#?}", res.status());
        //println!("{:#?}", send);
        

    let response = res.json::<ChannelInformation>().await.expect("Couldn't turn the response body into a proper struct"); 
    println!("Response Body: {:#?}", response);
    match response.data {
        Some(so) => Ok(format!("The title is: {}", so[0].title)),

        None => Ok(format!("Tried to get the title, but got an \"{}\" error", response.error.unwrap()))

    }
        
} 

pub async fn get_game(credentials: Arc::<types::Config>) -> Result<std::string::String, Box<(dyn std::error::Error + Send + Sync + 'static)>> {

    let req_client = reqwest::Client::new();
    let res = req_client.get("https://api.twitch.tv/helix/channels")
        .bearer_auth(credentials.secret.oauth.to_owned())
        .header("Client-Id", credentials.secret.client_id.to_owned())
        .query(&[("broadcaster_id", get_broadcaster_id(credentials).await.unwrap())])
        .send()
        .await
        .expect("Couldn't get a response");
        //let response_status = res.status();
        //println!("Response Status: {:#?}", res.status());
        //println!("{:#?}", send);
        

    let response = res.json::<ChannelInformation>().await.expect("Couldn't turn the response body into a proper struct"); 
    //println!("Response Body: {:#?}", response);
    match response.data {
        Some(so) => if so[0].game_name.len() == 0 {
            Ok(format!("{} is in an empty category", so[0].broadcaster_name))
        } else {
            Ok(format!("{} is currently in the \"{}\" category", so[0].broadcaster_name, so[0].game_name))
        }
        
        None => Ok(format!("Tried to get the title, but got an \"{}\" error", response.error.unwrap()))
    }
        
}
