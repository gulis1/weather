use crate::api::weather::{CurrentWeatherData, HourlyWeatherData, DailyWeatherData};
use crate::components::home::WeatherType;



pub mod http;

/* 
    let onclick = async_callback!(content, async move {
            let response = http_get_string("https://danbooru.donmai.us/posts/6815541.json").await;
            content.set(response);
    });

    Expands to:

    let onclick = clone!([content], move |_| {
        spawn_local(clone!([content], async move {
            let response = http_get_string("https://danbooru.donmai.us/posts/6815541.json").await;
            content.set(response);
        }))
    });

*/

#[macro_export]
macro_rules! async_callback {
    
    ($clonables:tt, $expr:expr) => {
        clone!($clonables, move |_| {
            $crate::wasm_bindgen_futures::spawn_local(clone!($clonables, $expr));
        })
    };
}

pub fn get_current_weather_type(weather: &CurrentWeatherData) -> WeatherType{

    if weather.snowfall > 0.0 { WeatherType::Snow }
    else if weather.rain > 0.0  { WeatherType::Rain }
    else if weather.cloudcover > 30 { WeatherType::Clouds}
    else { WeatherType::Clear } 
}

pub fn get_hourly_weather_type(hourly: &HourlyWeatherData, ind: usize) -> WeatherType{

    if hourly.snowfall[ind] > 0.0 { WeatherType::Snow }
    else if hourly.precipitation_probability[ind] > 10  { WeatherType::Rain }
    else if hourly.cloudcover[ind] > 30 { WeatherType::Clouds}
    else { WeatherType::Clear } 
}

pub fn get_daily_weather_type(daily: &DailyWeatherData, ind: usize) -> WeatherType {


    if daily.precipitation_probability_max[ind] > 20 && daily.snowfall_sum[ind] > 0.0 { WeatherType::Snow }
    else if daily.precipitation_probability_max[ind] > 20  { WeatherType::Rain }
    else {
        match daily.weather_code[ind] {
        1..=3 => WeatherType::Clouds,
        _ => WeatherType::Clear
        }
    }
}

pub fn get_background_uri(weather_type: WeatherType, is_day: i8) -> String {

    let file = match (weather_type, is_day) {
        (WeatherType::Clouds, 1)    =>  "clouds_day",
        (WeatherType::Clouds, 0)    =>  "clouds_night",
        (WeatherType::Rain, 1)     =>  "rain_day",
        (WeatherType::Rain, 0)     =>  "rain_night",
        (WeatherType::Clear, 1)     =>  "clear_day",
        (WeatherType::Clear, 0)     =>  "clear_night",
        (WeatherType::Snow, _)     =>  "snowy",

        _ => unreachable!("Boolean is_day can only be 0 or 1.")
    };

    format!("/assets/bgs/{}.jpg", file)
}