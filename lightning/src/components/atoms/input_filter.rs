use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub name_input: String,
    pub vec_buttons: Vec<String>,
}

#[function_component(InputFilter)]
pub fn input_filter(props: &Props) -> Html {
    //props
    let name_input = props.name_input.clone();
    let vec_buttons = props.vec_buttons.to_vec();

    //state
    let hide_fields_state = use_state(|| false);
    let input_state = use_state(|| "select...".to_owned());

    //open_close_input

    let hide_fields_state_cloned = hide_fields_state.clone();
    let onclick_hiden = Callback::from(move |_| {
        let num = *hide_fields_state_cloned.clone();
        hide_fields_state_cloned.set(!num);
    });

    let vec_buttons_clone = vec_buttons.to_vec();
    let input_state_cloned = input_state.clone();
    let hide_fields_state_cloned = hide_fields_state.clone();
    html! {
        <div class="input_filter">
        <p>{name_input}</p>
        <button onclick = {onclick_hiden} class="button_input_g">{&*input_state}</button>
        <div class={if *hide_fields_state {"buttons_inputs_active"} else {"buttons_inputs_inactive"}}>

        {
            vec_buttons_clone.iter().map(|i| {
                let input_state_cloned = input_state_cloned.clone();
                let hide_fields_state_cloned = hide_fields_state_cloned.clone();
                let  i = i.clone();
                let ii = i.clone();
                html!{
                    <button onclick = { Callback::from(move |_| {
                        let i = i.clone();
                        input_state_cloned.set(i.clone());
                        let num = *hide_fields_state_cloned.clone();
                        hide_fields_state_cloned.set(!num);
                    })
                } class="button_input"> {ii} </button>}
            }).collect::<Html>()
        }

        </div>
    </div>
    }
}
