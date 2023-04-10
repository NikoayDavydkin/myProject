use stylist::Style;
use yew::prelude::*;

use crate::components::organisms::{
    articles_content::ArticlesContent, footer_suport_sponsored::FooterSS, header::Header,
};
const STYLE_INPUT: &str = include_str!("../styles/articles_content.css");
#[function_component(Articles)]
pub fn articles() -> Html {
    let stylesheet = Style::new(STYLE_INPUT).unwrap();

    html! {
        <div class={stylesheet}>
        <Header/>
        <ArticlesContent/>
        <FooterSS/>
     </div>
    }
}
