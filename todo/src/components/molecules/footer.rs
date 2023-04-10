use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub count: usize,
    pub clear_all: Callback<()>,
}

#[function_component(Footer)]
pub fn footer(props: &Props) -> Html {
    let clear_all_cloned = props.clear_all.clone();
    let onclick = Callback::from(move |_| clear_all_cloned.emit(()));
    html! {
        <div class="footer">
            <span>{"You have "}<span class="pendingTasks"></span>{format!("{} pending tasks ",props.count)} </span>
            <button onclick = {onclick}>{"Clear All"}</button>
          </div>
    }
}
