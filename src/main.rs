//------------Modules------------------------------------------
mod api;
//------------Crates-------------------------------------------
use std::fs;
use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::message::ServerMessage;
use twitch_irc::ClientConfig;
use twitch_irc::SecureTCPTransport;
use twitch_irc::TwitchIRCClient;
//use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

use crate::api::{helix, sanitization, translation, translationv2, types};

pub async fn parse_time(duration: Duration) -> Result<String, Box<dyn std::error::Error>> {
    let duration_seconds = duration.as_secs() % 60;
    let duration_minutes = (duration.as_secs() / 60) % 60;
    let duration_hours = (duration.as_secs() / 60) / 60;
    let duration_days = duration.as_secs() / (24 * 60 * 60);
    let mut time_result = Vec::new();

    if duration_days > 1 {
        time_result.push(format!("{} days ", duration_days));
    } else if duration_days == 1 { 
        time_result.push(format!("{} day ", duration_days));
    }

    if duration_hours > 1 {
        time_result.push(format!("{} hours ", duration_hours));
    } else if duration_hours == 1 {
        time_result.push(format!("{} hour ", duration_hours));
    }

    if duration_minutes > 1 {
        time_result.push(format!("{} minutes ", duration_minutes));
    } else if duration_minutes == 1 {
        time_result.push(format!("{} minute ", duration_minutes));
    }

    if duration_seconds > 1 {
        time_result.push(format!("{} seconds ", duration_seconds));
    } else if duration_seconds == 1 {
        time_result.push(format!("{} second ", duration_seconds));
    }

    Ok(time_result.into_iter().map(|i| i.to_string()).collect::<String>())
}

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

    let chname = credentials.secret.channel_name.to_owned();

    let config = ClientConfig::new_simple(StaticLoginCredentials::new(
        credentials.secret.login.to_owned(),
        Some(credentials.secret.oauth.to_owned()),
    ));
    let (mut incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

    // first thing you should do: start consuming incoming messages,
    // otherwise they will back up.
    let join_handle = tokio::spawn(async move {
        client.join(chname.to_owned());
        while let Some(message) = incoming_messages.recv().await {
            match message {
                ServerMessage::Privmsg(msg) => {
                    println!(
                        "(#{}) {}: {}",
                        msg.channel_login, msg.sender.name, msg.message_text
                    );
                    if msg.sender.id == "477589350" && msg.message_text == "pepegaSit nevermind" {
                        client
                            .privmsg(chname.to_owned(), "ApuApustaja Slapp slchbot".to_owned())
                            .await
                            .unwrap();
                    }

                    if msg.sender.id == "82008718" && msg.message_text == "pajaS 🚨 ALERT" {
                        client
                            .privmsg(chname.to_owned(), "/me pajaGIGA 🚨 ТРИВОГА".to_owned())
                            .await
                            .unwrap();
                    }

                    if msg.message_text.starts_with("k!") {
                        let args = msg
                            .message_text
                            .replacen("k! ", "", 1)
                            .replacen("k!", "", 1);
                        let cleanargs: Vec<&str> = args.split_whitespace().collect();

                        if cleanargs[0] == "ping" {
                            let ping_time = Instant::now();
                            let cpingtime = ping_time - run_time;

                            client.reply_to_privmsg(format!("🌲 Pong! The bot has been running for {} | Version: {}",
                    parse_time(cpingtime).await.unwrap(),
                    env!("CARGO_PKG_VERSION")
                    ), &msg).await.unwrap();
                        }
                        if cleanargs[0] == "say" {
                            if cleanargs.len() == 1 {
                                client
                                    .reply_to_privmsg(
                                        "🌲 I do not have anything to say".to_owned(),
                                        &msg,
                                    )
                                    .await
                                    .unwrap();
                            } else {
                                let say = sanitization::sanitize_text(
                                    cleanargs[1..].join(" ").to_owned(),
                                );
                                client
                                    .reply_to_privmsg(say.await.unwrap(), &msg)
                                    .await
                                    .unwrap();
                            }
                        }
                        if cleanargs[0] == "commands" {
                            client.reply_to_privmsg("🌲 The command prefix is k! The commands are: ping, say, translate, title".to_owned(), &msg).await.unwrap();
                        }

                        if cleanargs[0] == "translate" {
                            if cleanargs.len() == 1 {
                                client
                                    .reply_to_privmsg(
                                        "🌲 You did not put what to translate".to_owned(),
                                        &msg,
                                    )
                                    .await
                                    .unwrap();
                            } else if cleanargs[1].contains(">") == true {
                                let tlt: Vec<&str> = cleanargs[1].split(">").collect();
                                if tlt.len() == 1 {
                                    client
                                        .reply_to_privmsg(
                                            translationv2::translate_text(
                                                "auto".to_owned(),
                                                "en".to_owned(),
                                                cleanargs[1..].join(" "),
                                            )
                                            .await,
                                            &msg,
                                        )
                                        .await
                                        .unwrap();
                                } else {
                                    client
                                        .reply_to_privmsg(
                                            translationv2::translate_text(
                                                tlt[0].to_owned(),
                                                tlt[1].to_owned(),
                                                cleanargs[2..].join(" "),
                                            )
                                            .await,
                                            &msg,
                                        )
                                        .await
                                        .unwrap();
                                }
                            } else {
                                client
                                    .reply_to_privmsg(
                                        translationv2::translate_text(
                                            "auto".to_owned(),
                                            "en".to_owned(),
                                            cleanargs[1..].join(" "),
                                        )
                                        .await,
                                        &msg,
                                    )
                                    .await
                                    .unwrap();
                            }
                        }

                        if cleanargs[0] == "ltranslate" {
                            if cleanargs.len() == 1 {
                                client
                                    .reply_to_privmsg(
                                        "🌲 You did not put what to translate".to_owned(),
                                        &msg,
                                    )
                                    .await
                                    .unwrap();
                            } else if cleanargs[1].contains(">") == true {
                                let tlt: Vec<&str> = cleanargs[1].split(">").collect();
                                if tlt.len() == 1 {
                                    client
                                        .reply_to_privmsg(
                                            translation::translate_text(
                                                "auto".to_owned(),
                                                "en".to_owned(),
                                                cleanargs[1..].join(" "),
                                            )
                                            .await,
                                            &msg,
                                        )
                                        .await
                                        .unwrap();
                                } else {
                                    client
                                        .reply_to_privmsg(
                                            translation::translate_text(
                                                tlt[0].to_owned(),
                                                tlt[1].to_owned(),
                                                cleanargs[2..].join(" "),
                                            )
                                            .await,
                                            &msg,
                                        )
                                        .await
                                        .unwrap();
                                }
                            } else {
                                client
                                    .reply_to_privmsg(
                                        translation::translate_text(
                                            "auto".to_owned(),
                                            "en".to_owned(),
                                            cleanargs[1..].join(" "),
                                        )
                                        .await,
                                        &msg,
                                    )
                                    .await
                                    .unwrap();
                            }
                        }

                        if cleanargs[0] == "title" {
                            let title_credentials = credentials.clone();
                            client
                                .reply_to_privmsg(
                                    helix::get_title(title_credentials).await.unwrap(),
                                    &msg,
                                )
                                .await
                                .unwrap();
                        }

                        if cleanargs[0] == "game" {
                            let game_credentials = credentials.clone();
                            client
                                .reply_to_privmsg(
                                    helix::get_game(game_credentials).await.unwrap(),
                                    &msg,
                                )
                                .await
                                .unwrap();
                        }

                        /*if cleanargs[0] == "status"{
                                status::is_up(cleanargs[1].to_owned());
                        }*/
                    }
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
