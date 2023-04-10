pub mod components;
pub mod db;
pub mod pages;
pub mod route;

//import

use route::{switch, Route};
use yew::prelude::*;
use yew_router::BrowserRouter;
use yew_router::Switch;

//struct data

#[function_component(App)]
pub fn app() -> Html {
    html! {
      <div>
      <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
        </div>
    }
}
