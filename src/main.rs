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
use crate::api::types;
use crate::internal::{lingva_translate, ping, say, tucking, weather};
use configr::Config;
//use std::sync::Arc;
use directories::ProjectDirs;
use std::time::Instant;

#[tokio::main]
pub async fn main() {
    let run_time = Instant::now();

    let credentials = match types::Secret::load("treuksbot", true) {
        Ok(ok) => ok,
        Err(er) => {
            let location = ProjectDirs::from("io", "TreuKS", "treuksbot").unwrap();
            println!("Config file not found. Attempting to create an empty one.");
            fs::create_dir_all(location.config_dir()).unwrap();
            panic!("{}", er);
        }
    };

    let channel_name = credentials.channel_name.to_owned();

    let config = ClientConfig::new_simple(StaticLoginCredentials::new(
        credentials.login.to_owned(),
        Some(credentials.oauth.to_owned()),
    ));
    let (mut incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

    // first thing you should do: start consuming incoming messages,
    // otherwise they will back up.
    let join_handle = tokio::spawn(async move {
        match client.join(channel_name.to_owned()) {
            Ok(join) => join,
            Err(error) => panic!("Could not join the channel {:?}", error),
        }
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
                                    say::say(clean_args.len(), &clean_args, &msg.sender.name).await,
                                    &msg,
                                )
                                .await
                                .unwrap(),
                            "translate" => client
                                .reply_to_privmsg(
                                    lingva_translate::translate_text(
                                        clean_args.len(),
                                        &clean_args,
                                        &msg.sender.name,
                                    )
                                    .await,
                                    &msg,
                                )
                                .await
                                .unwrap(),
                            "weather" => client
                                .reply_to_privmsg(
                                    weather::get_weather(
                                        clean_args.len(),
                                        &clean_args,
                                        credentials.openweather_oauth.clone(),
                                        &msg.sender.name,
                                    )
                                    .await,
                                    &msg,
                                )
                                .await
                                .unwrap(),
                            "tuck" => client
                                .reply_to_privmsg(
                                    tucking::tuck(clean_args.len(), &clean_args, &msg.sender.name)
                                        .await,
                                    &msg,
                                )
                                .await
                                .unwrap(),
                            "truck" => client
                                .reply_to_privmsg(
                                    tucking::truck(clean_args.len(), &clean_args, &msg.sender.name)
                                        .await,
                                    &msg,
                                )
                                .await
                                .unwrap(),
                            _ => (),
                        };
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
