use stylist::Style;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub buttons_page: Callback<i32>,
}

#[function_component(NavigationButtons)]
pub fn navigation_buttons(props: &Props) -> Html {
    let buttons_page = props.buttons_page.clone();
    //style
    let stylesheet: &str = include_str!("../../styles/navigation.css");

    let style = Style::new(stylesheet).expect("Failed to create style");

    //state for buttons

    let vec_num_state = use_state(|| vec![1, 2, 3, 4, 5]);

    //left

    let left_state_active = use_state(|| false);

    //1-5

    let button_active = use_state(|| 1);

    //right

    let right_state_active = use_state(|| true);

    let output_state = use_state(|| false);

    let output_state_cloned = output_state.clone();
    if *output_state_cloned {
        let vec_pages = vec_num_state.to_vec();
        let active_page = *button_active.clone();
        let page = vec_pages[active_page - 1];
        buttons_page.emit(page);
        output_state_cloned.set(false);
    }

    //functions

    //left
    let output_state_cloned = output_state.clone();
    let vec_num_state_clone = vec_num_state.clone();
    let right_state_active_cloned = right_state_active.clone();
    let left_state_active_cloned = left_state_active.clone();
    let vec_num_state_cloned = vec_num_state.to_vec();
    let button_active_cloned = button_active.clone();
    let left_oncklick = Callback::from(move |_| {
        let num_active = *button_active_cloned.clone();
        let num_1 = vec_num_state_cloned[0];
        if num_active == 2 || num_active == 1 {
            if num_1 == 1 {
                left_state_active_cloned.set(false);
                button_active_cloned.set(1);
            } else {
                button_active_cloned.set(3);
                let mut vec_num_state_cloned = vec_num_state_cloned.to_vec();
                for i in &mut vec_num_state_cloned {
                    *i -= 2;
                }
                vec_num_state_clone.set(vec_num_state_cloned);
            }
        } else {
            button_active_cloned.set(num_active - 1);
        }
        right_state_active_cloned.set(true);
        output_state_cloned.set(true);
    });

    //1
    let output_state_cloned = output_state.clone();
    let vec_num_state_clone = vec_num_state.clone();
    let vec_num_state_cloned = vec_num_state.to_vec();
    let right_state_active_cloned = right_state_active.clone();
    let left_state_active_cloned = left_state_active.clone();
    let button_active_cloned = button_active.clone();
    let one_oncklick = Callback::from(move |_| {
        if vec_num_state_cloned[0] == 1 {
            button_active_cloned.set(1);
            left_state_active_cloned.set(false);
        } else {
            button_active_cloned.set(3);
            left_state_active_cloned.set(true);

            let mut vec_num_state_cloned = vec_num_state_cloned.to_vec();
            for i in &mut vec_num_state_cloned {
                *i -= 2;
            }
            vec_num_state_clone.set(vec_num_state_cloned);
        }
        right_state_active_cloned.set(true);
        output_state_cloned.set(true);
    });

    //2
    let output_state_cloned = output_state.clone();
    let right_state_active_cloned = right_state_active.clone();
    let left_state_active_cloned = left_state_active.clone();
    let button_active_cloned = button_active.clone();
    let two_oncklick = Callback::from(move |_| {
        button_active_cloned.set(2);
        left_state_active_cloned.set(true);
        right_state_active_cloned.set(true);
        output_state_cloned.set(true);
    });

    //3
    let output_state_cloned = output_state.clone();
    let right_state_active_cloned = right_state_active.clone();
    let left_state_active_cloned = left_state_active.clone();
    let button_active_cloned = button_active.clone();
    let three_oncklick = Callback::from(move |_| {
        button_active_cloned.set(3);
        left_state_active_cloned.set(true);
        right_state_active_cloned.set(true);
        output_state_cloned.set(true);
    });

    //4
    let output_state_cloned = output_state.clone();
    let right_state_active_cloned = right_state_active.clone();
    let left_state_active_cloned = left_state_active.clone();
    let button_active_cloned = button_active.clone();
    let four_oncklick = Callback::from(move |_| {
        button_active_cloned.set(4);
        left_state_active_cloned.set(true);
        right_state_active_cloned.set(true);
        output_state_cloned.set(true);
    });

    //5
    let output_state_cloned = output_state.clone();
    let vec_num_state_clone = vec_num_state.clone();
    let vec_num_state_cloned = vec_num_state.to_vec();
    let right_state_active_cloned = right_state_active.clone();
    let left_state_active_cloned = left_state_active.clone();
    let button_active_cloned = button_active.clone();
    let five_oncklick = Callback::from(move |_| {
        if vec_num_state_cloned[4] == 101 {
            button_active_cloned.set(5);
            right_state_active_cloned.set(false);
            left_state_active_cloned.set(true);
        } else {
            button_active_cloned.set(3);
            right_state_active_cloned.set(true);
            left_state_active_cloned.set(true);
            let mut vec_num_state_cloned = vec_num_state_cloned.to_vec();
            for i in &mut vec_num_state_cloned {
                *i += 2;
            }
            vec_num_state_clone.set(vec_num_state_cloned);
        }
        output_state_cloned.set(true);
    });

    //right
    let output_state_cloned = output_state.clone();
    let vec_num_state_clone = vec_num_state.clone();
    let left_state_active_cloned = left_state_active.clone();
    let right_state_active_cloned = right_state_active.clone();
    let vec_num_state_cloned = vec_num_state.to_vec();
    let button_active_cloned = button_active.clone();
    let right_oncklick = Callback::from(move |_| {
        let num_active = *button_active_cloned.clone();
        let num_5 = vec_num_state_cloned[4];
        if num_active == 4 || num_active == 5 {
            if num_5 == 101 {
                right_state_active_cloned.set(false);
                button_active_cloned.set(5);
            } else {
                button_active_cloned.set(3);
                let mut vec_num_state_cloned = vec_num_state_cloned.to_vec();
                for i in &mut vec_num_state_cloned {
                    *i += 2;
                }
                vec_num_state_clone.set(vec_num_state_cloned);
            }
        } else {
            button_active_cloned.set(num_active + 1);
        }
        left_state_active_cloned.set(true);
        output_state_cloned.set(true);
    });

    let cloned_vec_num_state = vec_num_state.clone();
    let vec_num = cloned_vec_num_state.to_vec();
    let one = vec_num[0];
    let two = vec_num[1];
    let three = vec_num[2];
    let four = vec_num[3];
    let five = vec_num[4];

    html! {
            <div class={style} >
        <div class="navigation">


        <button onclick = {left_oncklick} class={if *left_state_active {"navigation_button_active"} else {"navigation_button_inactive"}}>
            <i class="fa fa-angle-left">
            </i>
        </button>

        <button onclick = {one_oncklick} class = {if *button_active ==1  {"navigation_button_active"} else {"navigation_button_inactive"}}>{one}</button>
        <button onclick = {two_oncklick} class={if *button_active ==2   {"navigation_button_active"} else {"navigation_button_inactive"}}>{two}</button>
        <button onclick = {three_oncklick} class={if *button_active ==3   {"navigation_button_active"} else {"navigation_button_inactive"}}>{three}</button>
        <button onclick = {four_oncklick} class={if *button_active ==4   {"navigation_button_active"} else {"navigation_button_inactive"}}>{four}</button>
        <button onclick = {five_oncklick} class = {if *button_active ==5   {"navigation_button_active"} else {"navigation_button_inactive"}}>{five}</button>

        <button onclick = {right_oncklick} class = {if *right_state_active  {"navigation_button_active"} else {"navigation_button_inactive"}}>
            <i class="fa fa-angle-right">
            </i>
        </button>
    </div>
        </div>
        }
}
