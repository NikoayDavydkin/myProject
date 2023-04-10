use crate::components::molecules::header::Header;
use crate::components::molecules::input_field::InputField;
use crate::components::molecules::todo_list::TodoList;
use crate::db::{request_to_the_server_issues, Issue};
use crate::{components::molecules::footer::Footer, db::FunctionSelection};
use stylist::{yew::styled_component, Style};
use uuid::Uuid;
use yew::prelude::*;

//style

const STYLE_INPUT: &str = include_str!("../input.css");

#[styled_component(IssuesPage)]
pub fn issues_page() -> Html {
    //styles
    let stylesheet = Style::new(STYLE_INPUT).unwrap();

    //state todo and state count
    let state_todo: UseStateHandle<Vec<Issue>> = use_state(Vec::new);

    // set data
    let cloned_state_todo = state_todo.clone();
    request_to_the_server_issues(cloned_state_todo, FunctionSelection::QueryIssues);

    //fn create issue
    let onsubmit = {
        let cloned_state_todo = state_todo.clone();
        Callback::from(move |title: String| {
            request_to_the_server_issues(
                cloned_state_todo.clone(),
                FunctionSelection::CreateIssue(title),
            );
        })
    };

    // fn remove all issue
    let clear_all = {
        let cloned_state_todo = state_todo.clone();
        Callback::from(move |_| {
            request_to_the_server_issues(
                cloned_state_todo.clone(),
                FunctionSelection::RemoveAllIssues,
            );
        })
    };

    // fn remove issue
    let delete = {
        let cloned_state_todo = state_todo.clone();
        Callback::from(move |id: Uuid| {
            request_to_the_server_issues(
                cloned_state_todo.clone(),
                FunctionSelection::RemoveIssue(id),
            );
        })
    };

    html! {
    <div class={stylesheet}>
      <div class="wrapper_home">
        <Header/>
        <InputField onsubmit={onsubmit} />
        <TodoList delete = {delete} todo = {state_todo.clone()}/>
        <Footer  clear_all={clear_all} count = {state_todo.len()}/>
      </div>
    </div>

        }
}
