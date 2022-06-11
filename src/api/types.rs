use configr::{Config, Configr};
use serde::{Deserialize, Serialize};

#[derive(Configr, Deserialize, Serialize, Default)]
pub struct Secret {
    pub owner: String,
    pub login: String,
    pub oauth: String,
    pub client_id: String,
    pub client_secret: String,
    pub channel_name: String,
    pub openweather_oauth: String,
}
