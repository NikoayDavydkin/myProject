use crate::{components::atoms::cart_product::CartProduct, db::Product};
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub state_products: UseStateHandle<Vec<Product>>,
}

#[function_component(CartsProducts)]
pub fn carts_products(props: &Props) -> Html {
    let state_products = props.state_products.clone();
    html! {
        <div class="carts_products">

            {
                state_products.iter().map(|product| {
                    let product = product.clone();
                    html!{
                        <CartProduct product = {product.clone()}/>
                     }
                }).collect::<Html>()
            }
        </div>

    }
}
