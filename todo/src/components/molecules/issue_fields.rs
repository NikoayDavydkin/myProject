use yew::prelude::*;

use crate::db::Issue;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub state_todo: UseStateHandle<Issue>,
}

#[function_component(IssueFields)]
pub fn issue_fields(props: &Props) -> Html {
    let difficulty = props.state_todo.difficulty;
    html! {
        <>
         //title and dot
         <div class="dot_and_title_issue">
        <div class={color_big_dot(difficulty)}></div>
         <div class="title_issue">{props.state_todo.title.clone()}</div>
        </div>

         //description

         <div class="name_issue_fields">{"Description"}</div>
         <div class="description_issue">{props.state_todo.description.clone()}</div>
         </>
    }
}

pub fn color_big_dot(difficulty: i32) -> String {
    match difficulty {
        1 => "colored_dot_big_easy".to_owned(),
        2 => "colored_dot_big_medium".to_owned(),
        3 => "colored_dot_big_hard".to_owned(),
        _ => "colored_dot_big_none".to_owned(),
    }
}
