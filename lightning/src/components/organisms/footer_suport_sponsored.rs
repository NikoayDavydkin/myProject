use yew::prelude::*;

#[function_component(FooterSS)]
pub fn footer_s_s() -> Html {
    html! {

    <footer class="footer">

        <div class="content_footer">
            <span>{"Powered by"}</span>
            <img src="../../img/logo.svg" alt="logo"/>
        </div>

    </footer>


    }
}
