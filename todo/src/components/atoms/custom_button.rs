use yew::prelude::*;

#[function_component(CustomButton)]
pub fn custom_button() -> Html {
    html! {
        <button ><i class="fas fa-plus"></i></button>
    }
}
