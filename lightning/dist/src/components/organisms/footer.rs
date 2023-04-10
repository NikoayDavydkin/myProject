use stylist::Style;
use yew::prelude::*;

const STYLE_INPUT: &str = include_str!("../../styles/footer.css");

#[function_component(Footer)]
pub fn footer() -> Html {
    //styles

    let stylesheet = Style::new(STYLE_INPUT).unwrap();

    html! {
    <div class={stylesheet}>

    <footer class="footer">

        <div class="content_footer">
            <span>{"Powered by"}</span>
            <img src="../../img/logo.svg" alt="logo"/>
        </div>

    </footer>
     </div>

    }
}
