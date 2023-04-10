use gloo_console::log;
use wasm_bindgen::JsCast;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::window;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::components::atoms::input_filter::InputFilter;

#[function_component(Filters)]
pub fn filters() -> Html {
    //"SortBy: Featured"
    let sort_by_vec = vec![
        "SortBy: Featured".to_owned(),
        "Price: Low to High".to_owned(),
        "Price: High to Low".to_owned(),
        "Processor: Fast to Slow".to_owned(),
        "Processor: Speed Per $".to_owned(),
        "Graphics: Fast to Slow".to_owned(),
        "Graphics: Speed Per $".to_owned(),
    ];

    //Condition
    let condition_vec = vec![
        "Used".to_owned(),
        "New".to_owned(),
        "Certified - Refurbished".to_owned(),
        "New Other".to_owned(),
        "Open Box".to_owned(),
        "Excellent - Refurbished".to_owned(),
        "Very Good - Refurbished ".to_owned(),
        "Good - Refurbished".to_owned(),
    ];

    //Operating System
    let operating_system_vec = vec![
        "Windows 10 Home".to_owned(),
        "Windows 10 Pro".to_owned(),
        "Windows 11 Home".to_owned(),
        "Windows 11 Pro".to_owned(),
        "Chrome OS".to_owned(),
        "Windows".to_owned(),
        "Windows 7 Home".to_owned(),
        "Windows 7 Pro".to_owned(),
        "Windows 8 Home".to_owned(),
        "Windows 8 Pro".to_owned(),
        "Windows 1".to_owned(),
        "Windows 2".to_owned(),
        "Windows 20".to_owned(),
        "Windows 15".to_owned(),
        "Windows 6".to_owned(),
    ];

    html! {
         //filter
         <div class="filter_content">

         <h1 class="filter_content_title" > {"Search for your laptop"}</h1>


        <InputFilter name_input = {"SortBy: Featured".to_owned()} vec_buttons = {sort_by_vec}/>
        <InputFilter name_input = {"Condition".to_owned()} vec_buttons = {condition_vec}/>
        <InputFilter name_input = {"Operating System".to_owned()} vec_buttons = {operating_system_vec}/>

         //1 slider
         <div>
         <input onchange = {Callback::from(move|event:Event|{
                log!("a");
         })} id = "screen_size1" type="text" value={"0"}/>
         <input id = "screen_size2" type="text" value={"18"}/>
         </div>
         <div class="input_filter_slider">
             <p>{"Screen Size: 0 - 17.3 inch"}</p>
             <div class="filters_slider" id="screen_size"></div>
         </div>
         // 2 slider
         <div class="input_filter_slider">
             <p>{"Screen Resolution: 0×0 - 5120×2160"}</p>
             <div class="filters_slider" id="screen_resolution"></div>
         </div>
         //  3 slider
          <div class="input_filter_slider">
             <p>{"CPU PassMark Performance Estimate: 0 - 25410"}</p>
             <div class="filters_slider" id="cpu_passmark_performance_estimate"></div>
         </div>
         // 4 slider
         <div class="input_filter_slider">
             <p>{"RAM Size: 0 - 256 GB"}</p>
             <div class="filters_slider" id="ram_size"></div>
         </div>
         //5 slider
         <div class="input_filter_slider">
         <p>{"SSD Size: 0 - 8000 GB"}</p>
             <div class="filters_slider" id="ssd_size"></div>
         </div>
         //6 slider
         <div class="input_filter_slider">
             <p>{"HDD Size: 0 - 1000 GB"}</p>
             <div class="filters_slider" id="hdd_size"></div>
         </div>
          //7 slider
          <div class="input_filter_slider">
             <p>{"GPU PassMark Performance Estimate: 0 - 16189"}</p>
             <div class="filters_slider" id="gpu_passmark_performance_estimate"></div>
         </div>
         //8 slider
         <div class="input_filter_slider">
             <p>{"Price: $0 to $20000"}</p>
             <div class="filters_slider" id="price"></div>
         </div>
     </div>
    }
}
