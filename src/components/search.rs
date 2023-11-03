use yew::prelude::*;
use web_sys::HtmlInputElement;
use gloo_storage::{LocalStorage, Storage};
use yew_icons::{Icon, IconId};
use yew_router::prelude::*;

use crate::{api::geo::{search_location, GeoResult}, Route};


#[derive(Properties, PartialEq)]
pub struct LocationSelectorProps {
    pub on_location_selected: Callback<Option<GeoResult>>,
}


#[function_component]
pub fn LocationSelector() -> Html {

    // State
    let input_ref = use_node_ref();
    let navigator = use_navigator().unwrap();
    let search_results = use_state(Vec::new);
    
    let on_input = async_callback!([input_ref, search_results], async move {

        let query = input_ref.cast::<HtmlInputElement>().map(|input| input.value()).unwrap();
        let response = search_location(&query).await.unwrap();
        search_results.set(response.results);
    });
    
    html! {

        <div class="flex flex-col w-screen h-screen items-center p-4 gap-4"
            style={
                "background: linear-gradient(rgba(0, 0, 0, 0.5), rgba(0, 0, 0, 0.6)), url(/assets/bgs/Search.jpg); background-size: cover"
            }
        >
            
            <div class="w-full flex items-center gap-2 justify-between">

                <Link<Route> to={Route::Home} classes="w-1/12 h-fit text-2xl text-white">
                    <Icon icon_id={IconId::LucideArrowLeft} width={"4rem"} height={"4rem"}/>
                </Link<Route>>  

                <div class="w-9/12 flex items-center border-2 border-white border-solid rounded-2xl bg-white text-2xl p-4" >
                    <input class="w-full h-full rounded-2xl focus:outline-none" onchange={on_input} ref={input_ref}/>
                    <Icon icon_id={IconId::LucideSearch} width={"2rem"} height={"2rem"}/>
                </div>
            </div>


            <ul class="w-full text-white"> { 
                (*search_results).clone().into_iter()
                .map(|loc| {
                    html! { 
                        <li onclick={clone!([loc], clone!([navigator], move |_| {
                                LocalStorage::set("location", loc.clone()).expect("Failed to save location.");
                                navigator.replace(&Route::Home);
                            }))}
                            class="w-full flex justify-start sm:justify-center items-center text-3xl rounded-2xl border-2 border-solid border-transparent gap-4 cursor-pointer hover:bg-sky-950 active:bg-sky-950"
                        >

                            <img src={format!("https://flagsapi.com/{}/flat/64.png", loc.country_code)}/>
                            <p> {loc.name.clone()} </p> 
                        </li>
                    }
                })
                .collect::<Html>()
            } </ul>
        </div>
    }
}