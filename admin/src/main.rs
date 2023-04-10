use yew::prelude::*;
use yew_router::prelude::*;

use crate::page::Page;

mod attribute_value;
mod attributes;
mod components;
mod cpu;
mod ids;
mod page;
mod serde;
mod synth_bench_client;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(route: &Route) -> Html {
    match route {
        Route::Home => html! {
            html! {
                <Page />
            }
        },
        Route::NotFound => {
            html! {
                <div>
                    <h1>{"Error: 404 Not found"}</h1>
                </div>
            }
        }
    }
}

#[function_component(Main)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

fn main() {
    yew::start_app::<Main>();
}
