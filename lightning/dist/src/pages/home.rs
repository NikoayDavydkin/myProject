use crate::components::organisms::footer::Footer;
use crate::components::organisms::header::Header;
use crate::components::organisms::home_content::HomeContent;
use crate::components::organisms::navigation_buttons::NavigationButtons;
use yew::prelude::*;
use crate::db::{Detail, Product, Search, SelectOffer};
use gql_client::Client;
use uuid::Uuid;

#[function_component(Home)]
pub fn home() -> Html {

  let page_st = use_state(||1);

    //start state

    let state_products = use_state(|| {
        vec![
           
            Product{
                id: Uuid::new_v4(),
                title: "LOADING".to_owned(),
                imageUrl: "https://i.ebayimg.com/images/g/gw4AAOSwi7FjjW25/s-l1600.jpg".to_owned(),
                seller : "Ebay".to_owned(),
                buyUrl : "https://www.ebay.com/itm/234807686372?mkevt=1&mkcid=1&mkrid=711-53200-19255-0&campid=5337578710&toolid=20006&customid=CUSTOMID".to_owned(),
                updated : "2022-12-16T09:00:00+00:00".to_owned(),
                selectOffer: SelectOffer{
                   price: 26789,
                   shipping: 0,
                   url: "https://www.ebay.com/itm/234807686372?mkevt=1&mkcid=1&mkrid=711-53200-19255-0&campid=5337578710&toolid=20006&customid=CUSTOMID".to_owned()
                },
                details : vec![
                   Detail{
                       text: "Intel Core i7-5700HQ Processor".to_owned(),
                   },
                   Detail{
                        text: "CPU PassMark: 6001".to_owned(),
                    },

                ],
               },

        ]
    });

    //update state
    let page_st_cloned = page_st.clone();
    let state_products_cloned = state_products.clone();
    wasm_bindgen_futures::spawn_local(async move {

      let query = &format!(" 
      
      query{{
        search(
          query: \"\",
              category: \"dfe0c6a8-3b02-41d5-8eab-5375ba4bc063\",
              sortOrder: \"35c69ed2-b015-4d77-b521-58cdaeea2935\",
              filters: [],
              price: {{
             max: 2000000,
            min: 0
          }}
              page: {}
              pageSize: 60
        ){{
          content {{
           ... on Search {{
             products {{
               id
               title
               imageUrl
               selectOffer {{
                 price
                 shipping
                 url
               }}
               details {{
                 text
               }}
               imageUrl
               seller
               buyUrl
               updated
             }}
           }}
         }}
       }}
        }}
      
      
      
      ", *page_st_cloned)[..];
      
      let client = Client::new("https://bonfire.dealtech.com/graphql");
      let data = client.query::<Search>(query).await.unwrap().unwrap();
      let data = data.search.content.products;
      state_products_cloned.set(data);

  });

        //buttons upadate
        
        let buttons_page =  {
          let page_st_cloned = page_st.clone();
          Callback::from(move |page: i32| {
            let page = page;
            page_st_cloned.set(page);

          })
        } ;

        //

        let state_products_cloned = state_products.clone();
    html! {
    <div>
        <Header/>
        <HomeContent state_products={state_products_cloned} />
        <NavigationButtons buttons_page={buttons_page}/>
        <Footer/>
     </div>
    }
}
