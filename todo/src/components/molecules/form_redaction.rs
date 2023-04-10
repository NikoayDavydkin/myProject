use crate::{
    components::molecules::{
        footer_issue_redaction::FooterIssueRedaction, issue_fields_redaction::IssueFieldsRedaction,
    },
    db::Issue,
};
use std::ops::Deref;
use uuid::Uuid;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub state_todo: UseStateHandle<Issue>,
    pub id: Uuid,
    pub onsubmit: Callback<Issue>,
}

#[function_component(FormRedaction)]
pub fn footer(props: &Props) -> Html {
    let state: UseStateHandle<Issue> = use_state(|| Issue {
        id: props.id,
        title: props.state_todo.title.clone(),
        priority: props.state_todo.priority,
        difficulty: props.state_todo.difficulty,
        description: props.state_todo.description.clone(),
    });

    // let state:UseStateHandle<Issue> = props.state_todo.clone();

    //title
    let cloned_state = state.clone();
    let handle_onchange_title = Callback::from(move |title: String| {
        let mut data = cloned_state.deref().clone();
        data.title = title;
        cloned_state.set(data)
    });

    //priority
    let cloned_state = state.clone();
    let handle_onchange_priority = Callback::from(move |priority: i32| {
        let mut data = cloned_state.deref().clone();
        data.priority = priority;
        cloned_state.set(data)
    });

    //difficulty
    let cloned_state = state.clone();
    let handle_onchange_difficulty = Callback::from(move |difficulty: i32| {
        let mut data = cloned_state.deref().clone();
        data.difficulty = difficulty;
        cloned_state.set(data)
    });

    //description
    let cloned_state = state.clone();
    let handle_onchange_description = Callback::from(move |description: String| {
        let mut data = cloned_state.deref().clone();
        data.description = description;
        cloned_state.set(data)
    });

    //onsubmit

    let form_onsubmit = props.onsubmit.clone();
    let cloned_state = state;
    let onsubmit = Callback::from(move |event: FocusEvent| {
        let cloned_state = cloned_state.clone();
        event.prevent_default();
        let data = cloned_state.deref().clone();

        form_onsubmit.emit(data)
    });

    html! {
        <form onsubmit={onsubmit}>
            <IssueFieldsRedaction
              state_todo={props.state_todo.clone()}
              handle_onchange_title={handle_onchange_title}
              handle_onchange_priority={handle_onchange_priority}
              handle_onchange_difficulty={handle_onchange_difficulty}
              handle_onchange_description={handle_onchange_description}
              />
            <FooterIssueRedaction id={props.id}/>
        </form>
    }
}
