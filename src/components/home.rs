use chrono::Timelike;
use yew::prelude::*;
use yew_hooks::{use_async, UseAsyncHandle};
use yew_router::prelude::*;
use gloo_storage::{LocalStorage, Storage};
use yew_icons::{Icon, IconId};

use crate::Route;
use crate::api::geo::GeoResult;
use crate::api::weather::{CurrentWeatherData, get_todays_weather, HourlyWeatherData};
use crate::components::hourly::HourlyWidget;
use crate::utils::{get_current_weather_type, get_background_uri};
use crate::components::weather_icon::WeatherIcon;


#[derive(Copy, Clone, Debug, PartialEq)]
pub enum WeatherType {
    Clear,
    Clouds,
    Rain,
    Snow
}

impl std::fmt::Display for WeatherType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
struct HomeData {
    current_data: CurrentWeatherData,
    hourly_data: HourlyWeatherData
}

#[function_component]
pub fn Home() -> Html {  
    
    let loc = LocalStorage::get::<GeoResult>("location").ok();
    let current_weather: UseAsyncHandle<HomeData, ()> = use_async(clone!([loc], async move {
        
        if let Some(loc) = loc {
            let response = get_todays_weather(loc.latitude, loc.longitude).await;
            match response {
                Ok(mut weather) => {
                    let hour = chrono::Local::now().hour() as usize;
                    weather.hourly.temperature_2m[hour] = weather.current.temperature_2m;
                    
                    Ok(HomeData{
                        current_data: weather.current,
                        hourly_data: weather.hourly
                    })
                }
                Err(_) => Err(()) 
            }
        }
        else { Err(()) }
    }));

    // Only refresh the view once.
    if loc.is_some() && current_weather.error.is_none() && !current_weather.loading && current_weather.data.is_none() {
        current_weather.run();
    }

    html! {            
        <div 
            class="z-0 h-screen w-screen absolute flex flex-col gap-32 text-white items-center p-8" 
            style={

                let bg_uri = match &current_weather.data {
                    Some(weather) => get_background_uri(get_current_weather_type(&weather.current_data), weather.current_data.is_day),
                    None => get_background_uri(WeatherType::Clear, 0)
                };
                format!("background: linear-gradient(rgba(0, 0, 0, 0.5), rgba(0, 0, 0, 0.6)), url({}); background-size: cover", bg_uri)
            }
        >
            <Link<Route> to={Route::Search} classes="h-fit">
                <button class="z-10 flex gap-8 text-4xl justify-center items-center h-fit mt-8 p-3 w-min pr-4 border-2 rounded-2xl border-solid border-white active:text-sky-300 active:border-sky-300">
                    <p class="h-fit">{loc.as_ref().map(|loc| loc.name.as_str()).unwrap_or("Search")}</p>
                    <Icon icon_id={IconId::LucideSearch} width="2rem" height="2rem"/>
                </button>
            </Link<Route>>  

            if let Some(ref weather) = current_weather.data {
                
                <div class="flex flex-col gap-4 justify-center items-center">
                    <p class="text-5xl">{format!("{}°C", weather.current_data.temperature_2m)}</p>
                    <WeatherIcon weather_type={get_current_weather_type(&weather.current_data)} is_day={weather.current_data.is_day} size={"4rem"}/>
                </div>
                
                <HourlyWidget hourly_data={weather.hourly_data.clone()} current_data={weather.current_data.clone()}/>
                
            }
        </div>       
    }
}