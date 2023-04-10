use yew::prelude::*;

use crate::db::Product;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub product: Product,
}

#[function_component(CartProduct)]
pub fn cart_product(props: &Props) -> Html {
    let id = props.product.id;
    let title = props.product.title.clone();
    let image_url = props.product.imageUrl.clone();
    let seller = props.product.seller.clone();
    let buy_url = props.product.buyUrl.clone();
    let updated = props.product.updated.clone();
    let price = props.product.selectOffer.price;
    let price = price as f64;
    let price = price / 100.0;
    let shipping = props.product.selectOffer.shipping;
    let url = props.product.selectOffer.url.clone();
    let details = props.product.details.to_vec();

    //state for more info

    let state_info = use_state(|| false);

    let state_info_cloned_1 = state_info.clone();
    let state_info_cloned_2 = state_info.clone();

    html! {


        <div class="cart_product">
        <div class="cart_product_content">
        <div class="cart_product_content_for_img">
            <img src={image_url}/>
            </div>
            <div class="cart_product_content_span">
            {title}
            </div>

            <div class = {if *state_info{"hover_info_active"} else{"hover_info_inactive"} }>{"Product prices and availability are accurate as of the date/time
            indicated and are subject to change. Any price and availability information 
            displayed on Amazon at the time of purchase will apply to the purchase of this product.
            "}</div>

            <ul>
                {
                    details.iter().map(|detail| {
                        let text = detail.text.clone();
                        html!{
                            <li>{text}</li>
                        }
                    }).collect::<Html>()
                }
            </ul>

            <div class="price_product">
                <span class="price1">{"$"}{price}</span>
                <span class="price2">{"as of 2 hours ago"}
                    <a

                    onmouseover={Callback::from(move|_|{
                        let state_info_cloned = state_info_cloned_1.clone();
                        let result = *state_info_cloned.clone();
                        state_info_cloned.set(!result);
                    })}

                    onmouseout={
                        Callback::from(move|_|{
                            let state_info_cloned = state_info_cloned_2.clone();
                            let result = *state_info_cloned.clone();
                            state_info_cloned.set(!result);
                        })
                    }

                    >
                    {"More Info"}</a>
                </span>
            </div>
            <div class="view_on_ebay">
            <a href={buy_url}><button>{
                if seller == "Ebay"{
                "View on Ebay"
                } else{
                "View on Amazon"
                }
        }</button></a>
            </div>

        </div>
    </div>


    }
}
