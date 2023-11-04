
use chrono::Datelike;
use yew::prelude::*;

use crate::api::weather::DailyWeatherData;
use crate::components::{weather_icon::WeatherIcon, home::WeatherType};
use crate::utils::get_daily_weather_type;


#[derive(Properties, PartialEq)]
pub struct DailyProps {
    pub daily_data: DailyWeatherData
}


#[function_component]
pub fn DailyWidget(props: &DailyProps) -> Html {  
    
    let daily = &props.daily_data;
    html! {            
        
        <div class="w-full">
            <b class="text-xl mb-2">{"Daily weather"}</b>
            <ul>
            {
                (0..7).map(|i| {
                    
                    let weekday = if i == 0 { String::from("Today") }
                    else {
                        chrono::NaiveDate::parse_from_str(&daily.time[i], "%Y-%m-%d")
                        .expect("Bad date.")
                        .weekday()
                        .to_string()
                    };
                    let temp_min = daily.temperature_2m_min[i];
                    let temp_max = daily.temperature_2m_max[i];
                    let precipitation_probability = daily.precipitation_probability_max[i];
                    let weather_type = get_daily_weather_type(daily, i);

                    html! {
                        <li class="flex items-center gap-2 py-2 border-gray-800 border-solid border-b">

                            <p class="w-4/12 ">{weekday}</p>
                            <span class="w-5/12 flex items-center gap-2">
                                <WeatherIcon weather_type={weather_type} is_day={1} size={"2rem"}/>
                                if weather_type == WeatherType::Rain || weather_type == WeatherType::Snow {
                                    <p>{precipitation_probability.to_string() + "%"}</p>
                                }
                            </span>
                            <p class="w-4/12 text-sky-300">{temp_min.to_string() + "°"}</p>
                            <p class="w-4/12 text-orange-300">{temp_max.to_string() + "°"}</p>
                        </li>
                    }
            
                })
                .collect::<Html>()
            }
            </ul>
        </div>
    }
}