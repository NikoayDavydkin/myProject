use std::ops::Deref;

use gql_client::Client;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use yew::prelude::*;

pub const ENDPOINT: &str = "https://issue-db.dealtech.com/graphql";

//data

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Issues {
    pub issues: Vec<Issue>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, Eq, Ord, PartialOrd)]
pub struct Issue {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub difficulty: i32,
    pub priority: i32,
}

//enum data

pub enum FunctionSelection {
    QueryIssues,
    CreateIssue(String),
    RemoveIssue(Uuid),
    RemoveAllIssues,
}

pub enum FunctionSelectionIssue {
    QueryIssue,
    UpdateIssue(Uuid, String, String, i32, i32),
}

pub fn request_to_the_server_issues(
    cloned_state_todo: UseStateHandle<Vec<Issue>>,
    function_selection: FunctionSelection,
) {
    wasm_bindgen_futures::spawn_local(async move {
        let query = &match function_selection {
            FunctionSelection::QueryIssues => "
      query{
          issues{
            id
            description
            title
            difficulty
            priority
          }
        }
          "
            .to_owned(),
            FunctionSelection::CreateIssue(title) => {
                format!(
                    "
                mutation {{
                  createIssue(
                    title : \"{title}\",
                    description: \"none\", 
                    priority:1,
                    difficulty:1,
                    repositories:\"88c2fc4a-d7c2-4f6e-9dd0-fff30f73e375\"
                  )
                }}
                "
                )
            }
            FunctionSelection::RemoveIssue(id) => {
                format!(
                    "
              mutation{{
                removeIssue(id : \"{id}\")
              }}
              "
                )
            }
            FunctionSelection::RemoveAllIssues => "
            mutation{
              removeAllIssues
            }
            "
            .to_owned(),
        }[..];
        let client = Client::new(ENDPOINT);
        let data = client.query::<Issues>(query).await.unwrap().unwrap();
        let data = data.issues;
        cloned_state_todo.set(data);
    });
}

pub fn request_to_the_server_issue(
    cloned_state_todo: UseStateHandle<Issue>,
    function_selection: FunctionSelectionIssue,
    id: Uuid,
) {
    wasm_bindgen_futures::spawn_local(async move {
        let endpoint = "https://issue-db.dealtech.com/graphql";
        let query = &match function_selection {
            FunctionSelectionIssue::QueryIssue => "
               query{
                issues{
                  id
                  description
                  title
                  priority
                  difficulty
                }
              }"
            .to_owned(),
            FunctionSelectionIssue::UpdateIssue(id, title, description, difficulty, priority) => {
                let title = if title == *"loading" {
                    cloned_state_todo.deref().clone().title
                } else {
                    title
                };
                let description = if description == *"loading" {
                    cloned_state_todo.deref().clone().description
                } else {
                    description
                };
                let priority = if priority == 0 {
                    cloned_state_todo.deref().clone().priority
                } else {
                    priority
                };
                let difficulty = if difficulty == 0 {
                    cloned_state_todo.deref().clone().difficulty
                } else {
                    difficulty
                };
                format!(
                    "
        mutation {{
            updateIssue(
             id: \"{id}\"
             title: \"{title}\"
             description: \"{description}\"
             priority: {priority}
             difficulty: {difficulty} 
           )
         }}
        "
                )
            }
        }[..];

        let client = Client::new(endpoint);
        let data = client.query::<Issues>(query).await.unwrap().unwrap().issues;
        let vec_new: Vec<Issue> = data.into_iter().filter(|issue| issue.id == id).collect();
        cloned_state_todo.set(vec_new[0].clone());
    });
}
