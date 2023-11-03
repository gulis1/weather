
use chrono::Timelike;
use yew::prelude::*;

use crate::{api::weather::{HourlyWeatherData, CurrentWeatherData}, components::{weather_icon::WeatherIcon, home::WeatherType}, utils::{get_hourly_weather_type, get_current_weather_type}};



#[derive(Properties, PartialEq)]
pub struct HourlyProps {
    pub hourly_data: HourlyWeatherData,
    pub current_data: CurrentWeatherData
}


#[function_component]
pub fn HourlyWidget(props: &HourlyProps) -> Html {  
    
    let hour = chrono::Local::now().hour() as usize;
    let hourly = &props.hourly_data;
    let current = &props.current_data;
    html! {            
        
        <div class="w-full box-border">
            <p class="text-xl pl-2">{"Hourly weather"}</p> 
            <ul class="w-full flex gap-1 overflow-scroll overflow-hidden pb-4">
            
            <li class="flex flex-col items-center p-2 gap-2">
                <p>{"Now"}</p>
                <WeatherIcon weather_type={get_current_weather_type(current)} is_day={current.is_day} size={"2rem"}/>
                <p>{current.temperature_2m.to_string() + "°"}</p>
            </li>

            {
                (hour + 1..hour+48)
                    .map(|i| {

                        let weather_type = get_hourly_weather_type(hourly, i);
                        html! {
                                         
                            <li class="flex flex-col items-center p-2 gap-2">
                                
                                if weather_type == WeatherType::Rainy || weather_type == WeatherType::Snowy {
                                        <p>{ format!("{:02}", i % 24) }</p>
                                    <div>

                                        <WeatherIcon weather_type={get_hourly_weather_type(hourly, i)} is_day={hourly.is_day[i]} size={"1.5rem"}/>
                                        <p class="text-xs">{hourly.precipitation_probability[i].to_string() + "%"}</p>
                                        <p>{hourly.temperature_2m[i].to_string() + "°"}</p>
                                    </div>
                                }

                                else {
                                    <p>{ format!("{:02}", i % 24) }</p>
                                    <WeatherIcon weather_type={get_hourly_weather_type(hourly, i)} is_day={hourly.is_day[i]} size={"2rem"}/>
                                    <p>{hourly.temperature_2m[i].to_string() + "°"}</p>
                                }
                                            
                            </li>
                        }
                    })
                    .collect::<Html>()
            }
            </ul>
        </div>
                  
    }
}