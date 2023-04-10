use crate::components::molecules::footer_issue::FooterIssue;
use crate::components::molecules::issue_fields::IssueFields;
use crate::db::{request_to_the_server_issue, FunctionSelectionIssue, Issue};
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

#[function_component(IssuePage)]
pub fn issue_page(props: &Props) -> Html {
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

    // set data

    let cloned_state_todo = state_todo.clone();
    request_to_the_server_issue(
        cloned_state_todo,
        FunctionSelectionIssue::QueryIssue,
        props.id,
    );

    //data

    html! {
        <div class={stylesheet}>
            <div class="wrapper_issue">
               <IssueFields state_todo={state_todo.clone()}/>
               <FooterIssue id={props.id}/>
            </div>
        </div>
    }
}
