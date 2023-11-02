use yew::prelude::*;
use yew_icons::{Icon, IconId};


use super::home::WeatherType;


#[derive(Properties, Clone, Debug, PartialEq)]
pub struct WeatherIconProps {
    pub weather_type: WeatherType,
    pub size: String,
    // Open meteo api returns 0 or 1 instead of true and false.
    pub is_day: i8
}

#[function_component]
pub fn WeatherIcon(props: &WeatherIconProps) -> Html {

    let icon_id = match (props.weather_type, props.is_day) {
        (WeatherType::Cloudy, _)    =>  IconId::LucideCloudy,
        (WeatherType::Rainy, _)     =>  IconId::LucideCloudRainWind,
        (WeatherType::Snowy, _)     =>  IconId::LucideSnowflake,
        (WeatherType::Clear, 1)  =>  IconId::LucideSun,
        (WeatherType::Clear, 0)  => IconId::LucideMoon,
        _ => unreachable!("Boolean is_day can only be 0 or 1.")
    };
      
    html!(<Icon icon_id={icon_id} width={props.size.clone()} height={props.size.clone()}/>)
}