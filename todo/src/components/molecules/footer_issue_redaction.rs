use uuid::Uuid;
use yew::prelude::*;
use yew_router::prelude::Link;

use crate::route::Route;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub id: Uuid,
}

#[function_component(FooterIssueRedaction)]
pub fn footer_issue_redaction(props: &Props) -> Html {
    let id = props.id;
    html! {
       <div class="footer_issue_redaction">
       <div class="right_redaction">
       <div  class="name_buttons_redaction">{"Save"}</div>
       <button id={"save_redaction"}><i class="fa fa-check"></i></button>
       <div  class="name_buttons_redaction">{"Exit"}</div>
       <Link<Route> to={Route::IssuePage {id}}>
       <button id={"exit_redaction"}><i class="fa fa-times"></i></button>
       </Link<Route>>
       </div>
       </div>
    }
}
