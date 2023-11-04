use gloo::net::http::Request;
use serde::Deserialize;
use anyhow::Result;


const API_URL: &str = "https://api.open-meteo.com/v1/forecast\
?current=temperature_2m,is_day,precipitation,rain,snowfall,cloudcover,weather_code\
&hourly=temperature_2m,snowfall,precipitation_probability,cloudcover,is_day,weather_code\
&daily=temperature_2m_max,temperature_2m_min,snowfall_sum,precipitation_probability_max,weather_code\
&timezone=auto\
&forecast_days=7";

#[derive(Debug, Clone, Deserialize)]
pub struct WeatherData {
    pub latitude: f64,
    pub longitude: f64,
    pub timezone: String,
    pub hourly: HourlyWeatherData,
    pub current: CurrentWeatherData,
    pub daily: DailyWeatherData
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct HourlyWeatherData {
    pub temperature_2m: Vec<f32>,
    pub precipitation_probability: Vec<i32>,
    pub snowfall: Vec<f32>,
    pub cloudcover: Vec<i32>,
    pub is_day: Vec<i8>,
    pub weather_code: Vec<i8>
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct DailyWeatherData {
    pub time: Vec<String>,
    pub temperature_2m_max: Vec<f32>,
    pub temperature_2m_min: Vec<f32>,
    pub precipitation_probability_max: Vec<i32>,
    pub snowfall_sum: Vec<f32>,
    pub weather_code: Vec<i8>
}


#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct CurrentWeatherData {
    pub temperature_2m: f32,
    pub rain: f32,
    pub snowfall: f32,
    pub cloudcover: i32,
    pub is_day: i8,
    pub weather_code: i8

}

pub async fn get_todays_weather(latitude: f64, longitude: f64) -> Result<WeatherData> {

    let response = Request::get(&format!("{}&latitude={}&longitude={}", API_URL, latitude, longitude))
            .send().await?
            .json::<WeatherData>().await?;

    Ok(response)    
}