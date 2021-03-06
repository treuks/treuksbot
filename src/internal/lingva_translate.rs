use crate::api::translate;

pub async fn translate_text(
    length: usize,
    clean_args_ref: &[&str],
    message_sender: &String,
) -> String {
    if length == 1 {
        "🌲 You did not put what to translate".to_owned()
    } else if clean_args_ref[1].contains('>') {
        let args_split_by_arrow: Vec<&str> = clean_args_ref[1].split('>').collect();
        if args_split_by_arrow.len() == 1 {
            let result = translate::translate_text(
                "auto".to_owned(),
                "en".to_owned(),
                clean_args_ref[1..].join(" "),
                message_sender,
            )
            .await;
            result
        } else {
            let result = translate::translate_text(
                args_split_by_arrow[0].to_owned(),
                args_split_by_arrow[1].to_owned(),
                clean_args_ref[2..].join(" "),
                message_sender,
            )
            .await;
            result
        }
    } else {
        let result = translate::translate_text(
            "auto".to_owned(),
            "en".to_owned(),
            clean_args_ref[1..].join(" "),
            message_sender,
        )
        .await;
        result
    }
}
