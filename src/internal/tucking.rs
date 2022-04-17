// TODO: add v2 sanitization
pub async fn tuck(length: usize, clean_args_ref: &[&str]) -> String {
    if length == 1 {
        "🌲 You didn't have anybody to tuck you in, so you tucked yourself in Sadge".to_owned()
    } else {
        format!(
            "🌲 You tucked {} into bed FeelsOkayMan 👉 🛏",
            clean_args_ref[1]
        )
    }
}

pub async fn truck(length: usize, clean_args_ref: &[&str]) -> String {
    if length == 1 {
        "🌲 Aww shucks, you got ran over KKona".to_owned()
    } else {
        format!(
            "🌲 You tucked {} into bed with a big, big truck KKona 👉 🛏",
            clean_args_ref[1]
        )
    }
}
