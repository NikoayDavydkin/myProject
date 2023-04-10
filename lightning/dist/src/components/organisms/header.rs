use gloo_console::log;
use stylist::Style;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::Link;

use crate::route::Route;

const STYLE_INPUT: &str = include_str!("../../styles/header.css");

#[function_component(Header)]
pub fn header() -> Html {
    let stylesheet = Style::new(STYLE_INPUT).unwrap();

    let button_state = use_state(|| false);

    let button_state_cloned = button_state.clone();
    let buttons_menu_clicked = Callback::from(move |_| {
        let num = *button_state_cloned.clone();
        button_state_cloned.set(!num)
    });

    //search

    let onchange_input = Callback::from(move |event: Event| {
        let target = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();
        log!(target)
    });

    html! {
        <div class={stylesheet}>
        <header class="header">

        <div class="content_header">

            <div class="logo">
                <img src="../../img/logo.svg" alt="logo"/>
            </div>
            <div class="search">
                <div class="search_content">
                    <span class="icon" >
                        <i class="fa fa-search">
                        </i>
                      </span>
                      //
                    <input onchange = {onchange_input} class="search_content_input" type="text" placeholder="Search for laptops.."/>
                </div>
            </div>
            <div class="menu">
            <Link<Route> to={Route::Home}>{"Home"}</Link<Route>>
            <Link<Route> to={Route::Articles}>{"Articles"}</Link<Route>>
            <Link<Route> to={Route::Support}>{"Support"}</Link<Route>>
            <Link<Route> to={Route::Sponsored}>{"Sponsored"}</Link<Route>>
            </div>
            <button onclick = {buttons_menu_clicked}class="menu_for_phone"><i class="fa fa-bars" aria-hidden="true"></i></button>

        </div>

    </header>
    <div class={if *button_state  {"buttons_menu_for_phone_active"} else {"buttons_menu_for_phone_inactive"}}>
    <Link<Route> to={Route::Home}><div>{"Home"}</div></Link<Route>>
    <Link<Route> to={Route::Articles}><div>{"Articles"}</div></Link<Route>>
    <Link<Route> to={Route::Support}><div>{"Support"}</div></Link<Route>>
    <Link<Route> to={Route::Sponsored}><div>{"Sponsored"}</div></Link<Route>>
    </div>

    </div>
    }
}
