use stylist::Style;
use yew::prelude::*;

use crate::components::organisms::{
    footer_suport_sponsored::FooterSS, header::Header, support_content::SupportContent,
};

const STYLE_INPUT: &str = include_str!("../styles/support_content.css");

#[function_component(Support)]
pub fn support() -> Html {
    let stylesheet = Style::new(STYLE_INPUT).unwrap();
    html! {


    <div class={stylesheet}>
    <div class="main_block_support">
        <Header/>
        <SupportContent/>
        </div>
        <FooterSS/>
     </div>
    }
}
