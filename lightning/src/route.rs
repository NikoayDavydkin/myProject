use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{support::Support, home::Home, articles::Articles, sponsored::Sponsored};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/support")]
    Support,
    #[at("/sponsored")]
    Sponsored,
    #[at("/articles")]
    Articles,
    #[not_found]
    #[at("/404")]
    NotFound,
}


pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {
            <>
             <Home/>
             <script src="../src/js/range_slider.js"></script>
             </>
            },
        Route::Sponsored => html! { <Sponsored/>},
        Route::Support => html! {<Support />},
        Route::Articles => html! {<Articles />},
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}