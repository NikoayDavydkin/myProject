use uuid::Uuid;
use yew::prelude::*;
use yew_router::prelude::Link;

use crate::route::Route;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: Uuid,
}

#[function_component(FooterIssue)]
pub fn footer_issue(props: &Props) -> Html {
    let id = props.id;

    html! {
       //FooterIssue

       <div class="footer_issue">
       <div class="right_issue">
       <div  class="name_buttons_issue">{"Home"}</div>
       <Link<Route> to={Route::Home}>
       <button id="home"><i class="fa fa-home"></i></button>
       </Link<Route>>
       <div  class="name_buttons_issue">{"Edit"}</div>
       <Link<Route> to={Route::IssuePageRedaction {id}}>
       <button id={"edit"}>
       <i class="fa fa-wrench">
       </i>
       </button>
       </Link<Route>>
       </div>
       </div>
    }
}
