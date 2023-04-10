use crate::components::atoms::custom_button::CustomButton;
use crate::components::atoms::custom_input::CustomInput;
use yew::prelude::*;

use std::ops::Deref;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub onsubmit: Callback<String>,
}

#[function_component(InputField)]
pub fn input_field(props: &Props) -> Html {
    let state = use_state(|| "none".to_owned());
    let cloned_state = state.clone();
    let handle_onchange = Callback::from(move |todo: String| {
        if todo == *"" {
            cloned_state.set("none".to_string())
        } else {
            cloned_state.set(todo)
        }
    });

    let form_onsubmit = props.onsubmit.clone();
    let onsubmit = Callback::from(move |event: FocusEvent| {
        let cloned_state = state.clone();
        event.prevent_default();
        let data = cloned_state.deref().clone();
        form_onsubmit.emit(data);
    });

    html! {
        <form onsubmit={onsubmit} class="set_todo_field">
        <CustomInput handle_onchange = {handle_onchange}/>
        <CustomButton />
        </form>
    }
}
