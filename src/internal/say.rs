use crate::api::sanitization;

pub async fn say(length: usize, clean_args_ref: &[&str], message_sender: &String) -> String {
    if length == 1 {
        "🌲 I do not have anything to say".to_owned()
    } else {
        let say = sanitization::sanitize_text_v2(clean_args_ref[1..].join(" "), Some(message_sender.to_string()));
        say.await.unwrap()
    }
}
