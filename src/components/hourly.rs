
use chrono::Timelike;
use yew::prelude::*;

use crate::{api::weather::HourlyWeatherData, components::weather_icon::WeatherIcon, utils::get_hourly_weather_type};



#[derive(Properties, PartialEq)]
pub struct HourlyProps {
    pub hourly_data: HourlyWeatherData,
}


#[function_component]
pub fn HourlyWidget(props: &HourlyProps) -> Html {  
    
    let hour = chrono::Local::now().hour() as usize;
    let hourly = &props.hourly_data;
    html! {            
        
        <div class="w-full box-border">
            <p class="text-xl pl-2">{"Hourly weather"}</p> 
            <ul class="w-full flex gap-1 overflow-scroll overflow-hidden pb-4">
            {
                (hour..hour+48)
                    .map(|i| {
                        html! {
                            <li class="flex flex-col items-center p-2 gap-2">
                                <p>{ if i == hour { String::from("Now") } else { format!("{:02}", i % 24) }}</p>
                                <WeatherIcon weather_type={get_hourly_weather_type(hourly, i)} is_day={hourly.is_day[i]} size={"2rem"}/>
                                <p>{hourly.temperature_2m[i].to_string() + "°"}</p>
                            </li>
                        }
                    })
                    .collect::<Html>()
            }
            </ul>
        </div>
                  
    }
}