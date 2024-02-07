use yew::prelude::*;
use yew_router::prelude::*;
use crate::app::Route;
use chrono::prelude::*;
#[derive(Properties, PartialEq)]
pub struct Props {
  pub change_page:Callback<String>,
}
#[function_component]
pub fn Sidebar() -> Html {
  
  let route: Route = use_route::<Route>().unwrap();
  // log!(serde_json::to_string_pretty(&route).unwrap());
  // let click = {
  //   let change_page = props.change_page.clone();
  //   move |e:MouseEvent| {
  //     // log!("clicked", e);
  //     // props.change_page.clone().emit("tert".to_owned());
  //     change_page.emit("Home".to_owned());
  //   }
  // };
  // props.change_page.emit("Home".to_owned());
  // let navigator = use_navigator().unwrap();
  // let onclick = Callback::from({
  //   let navigator = navigator.clone();
  //   move |_| navigator.push(&Route::Home)
  // });
  let current_date = chrono::Utc::now().date_naive();
  html! {
    <div class="flex flex-col p-2 bg-[#2e344e] h-screen text-xl justify-between">
      <div class="flex justify-center my-4 border-b border-gray-600 pb-8">
          <img src="./assets/images/logo.jpg" class="w-44 h-44 rounded-full border-2 border-white"/>
      </div>
      <div class="flex-1">
          <Link<Route> to={Route::Home}> 
            <div class={if route==Route::Home {"side_bar_box page_active"} else {"side_bar_box"}} >
              {"Home"} 
            </div>
          </Link<Route>>
          <Link<Route> to={Route::About}> 
            <div class={if route==Route::About {"side_bar_box page_active"} else {"side_bar_box"}} >
              {"About"}
            </div>
          </Link<Route>>
          <Link<Route> to={Route::Resume} >
            <div class={if route==Route::Resume {"side_bar_box page_active"} else {"side_bar_box"}}>
              {"Resume"}
            </div>
          </Link<Route>>
          <Link<Route> to={Route::Portfolio} >
            <div class={if route==Route::Portfolio {"side_bar_box page_active"} else {"side_bar_box"}}>
              {"Portfolio"}
            </div>
          </Link<Route>>
       
        <Link<Route> to={Route::Blog}> 
          <div class={if route==Route::Blog {"side_bar_box page_active"} else {"side_bar_box"}} >
            {"Blog"}
          </div>
        </Link<Route>>
        <div class="side_bar_box">
          {"Contact"}
        </div>
      </div>
      <div class="text-sm text-white text-center border-t border-gray-600 pt-4">
        { format!("@ {}. All Rights Reserved.", current_date.year()) }
      </div>
    </div>
  }
}