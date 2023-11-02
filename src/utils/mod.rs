use crate::api::weather::{CurrentWeatherData, HourlyWeatherData};
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

    if weather.snowfall > 0.0 { WeatherType::Snowy }
    else if weather.rain > 0.0  { WeatherType::Rainy }
    else if weather.cloudcover > 30 { WeatherType::Cloudy}
    else { WeatherType::Clear } 
}

pub fn get_hourly_weather_type(hourly: &HourlyWeatherData, ind: usize) -> WeatherType{

    if hourly.snowfall[ind] > 0.0 { WeatherType::Snowy }
    else if hourly.precipitation_probability[ind] > 30  { WeatherType::Rainy }
    else if hourly.cloudcover[ind] > 30 { WeatherType::Cloudy}
    else { WeatherType::Clear } 
}

pub fn get_background_uri(weather_type: WeatherType, is_day: i8) -> String {

    let file = match (weather_type, is_day) {
        (WeatherType::Cloudy, _)    =>  "cloudy",
        (WeatherType::Rainy, _)     =>  "rainy",
        (WeatherType::Snowy, _)     =>  "snowy",
        (WeatherType::Clear, 1)     =>  "clear_day",
        (WeatherType::Clear, 0)     =>  "clear_night",
        _ => unreachable!("Boolean is_day can only be 0 or 1.")
    };

    format!("/assets/bgs/{}.jpg", file)
}