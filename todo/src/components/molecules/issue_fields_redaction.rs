use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::db::Issue;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub state_todo: UseStateHandle<Issue>,
    pub handle_onchange_title: Callback<String>,
    pub handle_onchange_priority: Callback<i32>,
    pub handle_onchange_difficulty: Callback<i32>,
    pub handle_onchange_description: Callback<String>,
}

#[function_component(IssueFieldsRedaction)]
pub fn issue_fields_redaction(props: &Props) -> Html {
    let priority = props.state_todo.priority;
    let difficulty = props.state_todo.difficulty;

    let priority_errors = use_state(|| false);
    let cloned_priority_errors = priority_errors.clone();
    let difficulty_errors = use_state(|| false);
    let cloned_difficulty_errors = difficulty_errors.clone();
    //title
    let handle_onchange_title = props.handle_onchange_title.clone();
    let onchange_title = Callback::from(move |event: Event| {
        let value = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();
        handle_onchange_title.emit(value)
    });

    //priority
    let handle_onchange_priority = props.handle_onchange_priority.clone();
    let onchange_priority = Callback::from(move |event: Event| {
        let value = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value()
            .parse::<i32>()
            .unwrap();
        if value == 0 || value > 3 {
            cloned_priority_errors.set(true);
            handle_onchange_priority.emit(priority)
        } else {
            cloned_priority_errors.set(false);
            handle_onchange_priority.emit(value)
        }
    });

    //difficulty
    let handle_onchange_difficulty = props.handle_onchange_difficulty.clone();
    let onchange_difficulty = Callback::from(move |event: Event| {
        let value = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value()
            .parse::<i32>()
            .unwrap();
        if value == 0 || value > 3 {
            cloned_difficulty_errors.set(true);
            handle_onchange_difficulty.emit(difficulty)
        } else {
            cloned_difficulty_errors.set(false);
            handle_onchange_difficulty.emit(value)
        }
    });

    //description
    let handle_onchange_description = props.handle_onchange_description.clone();
    let onchange_description = Callback::from(move |event: Event| {
        let value = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();
        handle_onchange_description.emit(value)
    });

    html! {
        <>
         <div class="name_redaction_fields">{"Title"}</div>
         <input onchange={onchange_title} type="text" class="text_redaction_fields" placeholder={props.state_todo.title.clone()}/>
         <p class="errors_text_none">{"enter a number from 1-3"}</p>

         <div class="name_redaction_fields">{"Priority"}</div>
         <input onchange={onchange_priority} type="text"  class={if !*priority_errors {
            "text_redaction_fields"
         }else{
            "text_redaction_fields_errors"
         } } placeholder = {priority.to_string()}/>
         <p class={if !*priority_errors {
            "errors_text_none"
         }else{
            "errors_text"
         } }>{"enter a number from 1-3"}</p>

         <div class="name_redaction_fields">{"Difficulty"}</div>
         <input onchange={onchange_difficulty}  type="text"  class={if !*difficulty_errors {
            "text_redaction_fields"
         }else{
            "text_redaction_fields_errors"
         } } placeholder={difficulty.to_string()}/>
         <p class={if !*difficulty_errors {
            "errors_text_none"
         }else{
            "errors_text"
         } }>{"enter a number from 1-3"}</p>

         <div class="name_redaction_fields">{"Description"}</div>
         <textarea onchange={onchange_description} type="text"  class="description_redaction_fields" placeholder={props.state_todo.description.clone()}/>

         </>
    }
}
