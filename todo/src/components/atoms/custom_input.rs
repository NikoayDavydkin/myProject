use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub handle_onchange: Callback<String>,
}

#[function_component(CustomInput)]
pub fn custom_input(props: &Props) -> Html {
    let handle_onchange = props.handle_onchange.clone();
    let onchange = Callback::from(move |event: Event| {
        let value = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();

        handle_onchange.emit(value)
    });
    html! {
        <input onchange={onchange} type="text" placeholder="Add your new todo"/>
    }
}
