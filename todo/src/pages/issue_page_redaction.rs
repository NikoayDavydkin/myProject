use crate::{
    components::molecules::form_redaction::FormRedaction,
    db::{request_to_the_server_issue, FunctionSelectionIssue, Issue},
};

use stylist::Style;
use uuid::Uuid;
use yew::prelude::*;

//style

pub const STYLE_INPUT: &str = include_str!("../input.css");

//props

#[derive(PartialEq, Properties)]
pub struct Props {
    pub id: Uuid,
}

//data

#[function_component(IssuePageRedaction)]
pub fn issue_page_redaction(props: &Props) -> Html {
    //style
    let stylesheet = Style::new(STYLE_INPUT).unwrap();

    //state todo
    let state_todo: UseStateHandle<Issue> = use_state(|| Issue {
        id: Uuid::new_v4(),
        title: "loading".to_owned(),
        description: "loading".to_owned(),
        priority: 0,
        difficulty: 0,
    });

    let id = props.id;

    // set data
    let cloned_state_todo = state_todo.clone();
    request_to_the_server_issue(
        cloned_state_todo,
        FunctionSelectionIssue::QueryIssue,
        props.id,
    );

    //onsubmit

    let onsubmit = {
        let cloned_state_todo = state_todo.clone();
        Callback::from(move |new_issue: Issue| {
            request_to_the_server_issue(
                cloned_state_todo.clone(),
                FunctionSelectionIssue::UpdateIssue(
                    id,
                    new_issue.title,
                    new_issue.description,
                    new_issue.difficulty,
                    new_issue.priority,
                ),
                id,
            );
        })
    };

    html! {
        <div class={stylesheet}>
            <div class="wrapper_redaction">
            <FormRedaction onsubmit={onsubmit} state_todo={state_todo.clone()} id={id} />
            </div>
        </div>
    }
}
