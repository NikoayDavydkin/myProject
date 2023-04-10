use crate::pages::issue_page::IssuePage;
use crate::pages::issue_page_redaction::IssuePageRedaction;
use crate::pages::issues_page::IssuesPage;
use uuid::Uuid;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/:id")]
    IssuePage { id: Uuid },
    #[at("/:id/redaction")]
    IssuePageRedaction { id: Uuid },
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: &Route) -> Html {
    match route {
        Route::Home => html! {
            html! {
                <IssuesPage />
            }
        },
        Route::IssuePage { id } => html! {
            html! {
                <IssuePage
                    id={*id}
                />
            }
        },
        Route::IssuePageRedaction { id } => html! {
            html! {
                <IssuePageRedaction
                    id={*id}
                />
            }
        },
        Route::NotFound => {
            html! {
                <div>
                    <h1>{"Error: 404 Not found"}</h1>
                </div>
            }
        }
    }
}
