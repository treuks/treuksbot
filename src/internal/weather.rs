use crate::api::weather;
// TODO: add v2 sanitization
pub async fn get_weather(
    length: usize,
    clean_args_ref: &[&str],
    open_weather_oauth: String,
) -> String {
    if length == 1 {
        "🌲 You did not enter a region".to_owned()
    } else {
        let weather_result =
            weather::get_weather(clean_args_ref[1..].join(" "), open_weather_oauth).await;
        match weather_result {
            Ok(response) => match response.sys.country {
                Some(countryexists) => format!(
                    "🌲 The weather in {}/{}, is: {} | Temperature is {}°C | Feels like {}°C ",
                    response.name,
                    countryexists,
                    response.weather[0].main,
                    response.main.temp,
                    response.main.feels_like,
                ),
                None => format!(
                    "🌲 The weather in {} is: {} | Temperature is {}°C | Feels like {}°C ",
                    response.name,
                    response.weather[0].main,
                    response.main.temp,
                    response.main.feels_like,
                ),
            },
            Err(e) => format!("{}", e),
        }
    }
}
