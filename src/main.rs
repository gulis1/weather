#[macro_use]
extern crate clone_macro;
pub extern crate wasm_bindgen_futures;


#[macro_use]
pub mod utils;
pub mod api;
pub mod components;

use components::{home::Home, search::LocationSelector};
use yew_router::prelude::*;
use yew::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/search")]
    Search,
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html!(<Home/>),
        Route::Search  => html!(<LocationSelector/>)
    }
}

#[function_component(Main)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<Main>::new().render();
}