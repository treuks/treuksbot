//------------Modules------------------------------------------
mod api;
mod internal;
//------------Crates-------------------------------------------
use std::fs;
use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::message::ServerMessage;
use twitch_irc::ClientConfig;
use twitch_irc::SecureTCPTransport;
use twitch_irc::TwitchIRCClient;
//use std::collections::HashMap;
use crate::api::{helix, types, weather};
use crate::internal::{lingva_translate, ping, say};
use std::sync::Arc;
use std::time::Instant;

#[tokio::main]
pub async fn main() {
    let run_time = Instant::now();
    let file_location = format!("{}/.config/treuksbot/secret.toml", env!("HOME"));
    // let file = fs::read_to_string(file_location).expect("I couldn't find the file");
    // let credentials = Arc::<types::Config>::new(toml::from_str(&file).expect("I couldn't find any credentials, is the file filled out correctly?"));

    // let token_clone = credentials.clone();
    // helix::check_oauth_token(token_clone, file_location.to_owned()).await;

    let file = fs::read_to_string(file_location).expect("I couldn't find the file");
    let credentials = Arc::<types::Config>::new(
        toml::from_str(&file)
            .expect("I couldn't find any credentials, is the file filled out correctly?"),
    );

    let channel_name = credentials.secret.channel_name.to_owned();

    let config = ClientConfig::new_simple(StaticLoginCredentials::new(
        credentials.secret.login.to_owned(),
        Some(credentials.secret.oauth.to_owned()),
    ));
    let (mut incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

    // first thing you should do: start consuming incoming messages,
    // otherwise they will back up.
    let join_handle = tokio::spawn(async move {
        client.join(channel_name.to_owned());
        while let Some(message) = incoming_messages.recv().await {
            match message {
                ServerMessage::Privmsg(msg) => {
                    println!(
                        "(#{}) {}: {}",
                        msg.channel_login, msg.sender.name, msg.message_text
                    );
                    if msg.sender.id == "477589350" && msg.message_text == "pepegaSit nevermind" {
                        client
                            .privmsg(
                                channel_name.to_owned(),
                                "ApuApustaja Slapp slchbot".to_owned(),
                            )
                            .await
                            .unwrap();
                    }

                    if msg.sender.id == "82008718" && msg.message_text == "pajaS 🚨 ALERT" {
                        client
                            .privmsg(
                                channel_name.to_owned(),
                                "/me pajaGIGA 🚨 ТРИВОГА".to_owned(),
                            )
                            .await
                            .unwrap();
                    }

                    if msg.message_text.starts_with("k!") {
                        let args = msg
                            .message_text
                            .replacen("k! ", "", 1)
                            .replacen("k!", "", 1);
                        let clean_args: Vec<&str> = args.split_whitespace().collect();

                        match clean_args[0] {
                            "ping" => client
                                .reply_to_privmsg(ping::ping(run_time).await, &msg)
                                .await
                                .unwrap(),
                            "say" => client
                                .reply_to_privmsg(
                                    say::say(clean_args.len(), &clean_args).await,
                                    &msg,
                                )
                                .await
                                .unwrap(),
                            "translate" => client
                                .reply_to_privmsg(
                                    lingva_translate::translate_text(clean_args.len(), &clean_args)
                                        .await,
                                    &msg,
                                )
                                .await
                                .unwrap(),
                            _ => (),
                        };

                        if clean_args[0] == "title" {
                            let title_credentials = credentials.clone();
                            client
                                .reply_to_privmsg(
                                    helix::get_title(title_credentials).await.unwrap(),
                                    &msg,
                                )
                                .await
                                .unwrap();
                        }

                        if clean_args[0] == "game" {
                            let game_credentials = credentials.clone();
                            client
                                .reply_to_privmsg(
                                    helix::get_game(game_credentials).await.unwrap(),
                                    &msg,
                                )
                                .await
                                .unwrap();
                        }

                        if clean_args[0] == "tuck" {
                            if clean_args.len() == 1 {
                                client
                                    .reply_to_privmsg(
                                        "🌲 You didn't have anybody to tuck you in, so you tucked yourself in Sadge".to_owned(),
                                        &msg,
                                    )
                                    .await
                                    .unwrap();
                            } else {
                                client
                                    .privmsg(
                                        channel_name.to_owned(),
                                        format!(
                                            "🌲 You tucked {} into bed FeelsOkayMan 👉 🛏",
                                            clean_args[1]
                                        ),
                                    )
                                    .await
                                    .unwrap();
                            }
                        }

                        if clean_args[0] == "truck" {
                            if clean_args.len() == 1 {
                                client
                                    .reply_to_privmsg(
                                        "🌲 Aww shucks, you got ran over KKona".to_owned(),
                                        &msg,
                                    )
                                    .await
                                    .unwrap();
                            } else {
                                client
                                    .privmsg(channel_name.to_owned(),
                                    format!("🌲 You tucked {} into bed with a big, big truck KKona 👉 🛏", clean_args[1])
                                )
                                    .await
                                    .unwrap();
                            }
                        }

                        if clean_args[0] == "weather" {
                            if clean_args.len() == 1 {
                                client
                                    .reply_to_privmsg(
                                        "🌲 You did not enter a region".to_owned(),
                                        &msg,
                                    )
                                    .await
                                    .unwrap();
                            } else {
                                let open_weather_map_credentials =
                                    credentials.secret.openweather_oauth.clone();
                                let weather_result = weather::get_weather(
                                    clean_args[1..].join(" ").to_string(),
                                    open_weather_map_credentials,
                                )
                                .await;
                                match weather_result {
                                    Ok(response) =>
                                    match response.sys.country {
                                        Some(countryexists) =>
                                        client.reply_to_privmsg(format!("🌲 The weather in {}/{}, is: {} | Temperature is {}°C | Feels like {}°C ",
                                        response.name,
                                        countryexists,
                                        response.weather[0].main,
                                        response.main.temp,
                                        response.main.feels_like,
                                    ),
                                        &msg,
                                    ).await.unwrap(),
                                    None =>
                                            client.reply_to_privmsg(format!("🌲 The weather in {} is: {} | Temperature is {}°C | Feels like {}°C ",
                                            response.name,
                                            response.weather[0].main,
                                            response.main.temp,
                                            response.main.feels_like,
                                        ),
                                            &msg,
                                        ).await.unwrap()

                                    },
                                    Err(e) => client.reply_to_privmsg(format!("{}", e), &msg).await.unwrap()
                                }
                            }
                        }
                    };
                }
                ServerMessage::Whisper(msg) => {
                    println!("(w) {}: {}", msg.sender.name, msg.message_text);
                }
                _ => {}
            }
        }
    });

    // join the channel

    // keep the tokio executor alive.
    // If you return instead of waiting the background task will exit.
    join_handle.await.unwrap();
}
