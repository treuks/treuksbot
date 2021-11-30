use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub secret: Secret,
}

#[derive(Debug, Deserialize)]
pub struct Secret {
    pub owner: String,
    pub login: String,
    pub oauth: String,
    pub client_id: String,
    pub client_secret: String,
    pub channel_name: String,
}

impl Config {
    pub fn new(secret: Secret) -> Self {
        Config { secret }
    }
}
