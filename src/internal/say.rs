use crate::api::sanitization;

pub async fn say(length: usize, clean_args_ref: &Vec<&str>) -> String {
    if length == 1 {
        "🌲 I do not have anything to say".to_owned()
    } else {
        let say = sanitization::sanitize_text(clean_args_ref[1..].join(" ").to_owned());
        say.await.unwrap()
    }
}
