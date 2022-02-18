use humantime::format_duration;
use std::time::Instant;

pub async fn ping(run_time: Instant) -> String {
    let ping_time = Instant::now();
    let cpingtime = ping_time - run_time;

    format!(
        "🌲 Pong! The bot has been running for {} | Version: {}",
        format_duration(cpingtime),
        env!("CARGO_PKG_VERSION")
    )
}
