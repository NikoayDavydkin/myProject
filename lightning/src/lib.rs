mod components;
mod pages;
mod route;
mod db;

use crate::route::switch;
use crate:: route::Route;
use yew::prelude::*;
use yew_router::{BrowserRouter, Switch};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
        <BrowserRouter>
            <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
        </BrowserRouter>
        <script src="./src/js/range_slider.js"></script>
        </>
    }
}
