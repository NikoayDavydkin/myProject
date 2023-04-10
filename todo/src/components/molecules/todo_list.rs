use crate::db::Issue;
use crate::Route;
use uuid::Uuid;
use yew::prelude::*;
use yew_router::prelude::Link;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub todo: UseStateHandle<Vec<Issue>>,
    pub delete: Callback<Uuid>,
}

#[function_component(TodoList)]
pub fn todo_list(props: &Props) -> Html {
    let mut vec_todo = props.todo.to_vec();
    vec_todo.sort_by(|a, b| a.priority.cmp(&b.priority));
    html! {
        <ul class="todoList">
        {
            vec_todo.iter().map(|task| {
              let id = task.id;
              let title = task.title.clone();
              let delete = props.delete.clone();
              let difficulty = task.difficulty;
                html!{<li><div class={color_small_dot(difficulty)}></div>
                  <Link<Route> to={Route::IssuePage {id}}>
                    {title}
                    </Link<Route>>
                       <button  onclick = {Callback::from(move|_| delete.emit(id) )} >
                         <span class="icon" >
                           <i class="fas fa-trash">
                           </i>
                         </span>
                       </button>
                     </li>}
            }).collect::<Html>()
        }
      </ul>
    }
}

pub fn color_small_dot(difficulty: i32) -> String {
    match difficulty {
        1 => "colored_dot_small_easy".to_owned(),
        2 => "colored_dot_small_medium".to_owned(),
        3 => "colored_dot_small_hard".to_owned(),
        _ => "colored_dot_small_none".to_owned(),
    }
}
