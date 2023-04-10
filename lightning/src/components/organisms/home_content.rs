use crate::components::molecules::carts_products::CartsProducts;
use crate::components::molecules::filters::Filters;
use crate::db::Product;
use stylist::Style;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub state_products: UseStateHandle<Vec<Product>>,
}

const STYLE_INPUT: &str = include_str!("../../styles/home_content.css");

const STYLE_INPUT_2: &str = include_str!("../../styles/nouislider.min.css");

#[function_component(HomeContent)]
pub fn home_content(props: &Props) -> Html {
    let stylesheet = Style::new(STYLE_INPUT).unwrap();
    let stylesheet_2 = Style::new(STYLE_INPUT_2).unwrap();

    let state_products = props.state_products.clone();

    html! {
        <div class={stylesheet}>
        <div class={stylesheet_2}>
        <div class="content">
        <Filters/>
        <CartsProducts state_products={state_products}/>
        </div>
        </div>
        </div>

    }
}
