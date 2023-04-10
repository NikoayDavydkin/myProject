use yew::prelude::*;

#[function_component(SupportContent)]
pub fn support_content() -> Html {
    html! {
        <div class="support_content">
        <div class="support_name">
        <h1>{"Get in touch"}</h1>
        <p>{"Have an inquiry or some feedback for us?"}</p>
        </div>
        <div class="support_adress">
        <p>{"Email us at | "} <a href= "mailto:support@dealtech.com">{"support@dealtech.com"}</a></p>
        <p>{"Or write us | P.O. Box 50802, Mesa AZ, 85208"}</p>
        </div>
        </div>
    }
}
