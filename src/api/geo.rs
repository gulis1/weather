use anyhow::Result;
use serde::{Deserialize, Serialize};
use gloo::net::http::Request;

// TODO: make language customizable.
const API_URL: &str = "https://geocoding-api.open-meteo.com/v1/search?count=10&language=en&format=json&language=es&name=";


#[derive(Clone, Debug, Deserialize)]
pub struct GeoResults {
    pub results: Vec<GeoResult>
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct GeoResult {
    pub id: i32,
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub country_code: String,
    pub timezone: String
}

pub async fn search_location(name: &str) -> Result<GeoResults> {

    let call = format!("{}{}", API_URL, name);
    let response = Request::get(&call).send().await?
        .json::<GeoResults>().await?;

    Ok(response)
}