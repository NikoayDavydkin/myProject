use stylist::Style;
use yew::prelude::*;

const STYLE_INPUT: &str = include_str!("../styles/sponsored_content.css");

use crate::components::organisms::{footer_suport_sponsored::FooterSS, header::Header};

#[function_component(Sponsored)]
pub fn sponsored() -> Html {
    let stylesheet = Style::new(STYLE_INPUT).unwrap();
    html! {
    <>
        <Header/>
        <div class={stylesheet}>
        <div class="sponsored_content">
        {"As an Amazon Associate, DealTech earns from qualifying purchases.
         DealTech may recieve a commission if you purchase something though one of our links.
        Certain content that appears on DealTech comes from Amazon.
        This content is provided `as is` and is subject to change or removal at any time."}
        </div>
        <FooterSS/>
        </div>

     </>
    }
}
