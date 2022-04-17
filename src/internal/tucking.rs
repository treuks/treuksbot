use crate::api::sanitization;
pub async fn tuck(length: usize, clean_args_ref: &[&str], message_sender: &String) -> String {
    if length == 1 {
        "🌲 You didn't have anybody to tuck you in, so you tucked yourself in Sadge".to_owned()
    } else {
        sanitization::sanitize_text_v2(
            format!(
                "You tucked {} into bed FeelsOkayMan 👉 🛏",
                clean_args_ref[1]
            ),
            Some(message_sender.to_string()),
        )
        .await
        .unwrap()
    }
}

pub async fn truck(length: usize, clean_args_ref: &[&str], message_sender: &String) -> String {
    if length == 1 {
        "🌲 Aww shucks, you got ran over KKona".to_owned()
    } else {
        sanitization::sanitize_text_v2(
            format!(
                "You tucked {} into bed with a big, big truck KKona 🚚 👉 🛏",
                clean_args_ref[1]
            ),
            Some(message_sender.to_string()),
        )
        .await
        .unwrap()
    }
}
