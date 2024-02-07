use gloo::console::log;
use serde::{Deserialize, Serialize};
use yew::prelude::*;
use crate::components::services::services::get_data;


#[derive(Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
struct Post {
  user_id:i32,
  id:i32,
  title:String,
  body:String,
}

#[derive(Properties, PartialEq)]
pub struct BlogProps {
  pub id:String
}
#[function_component]
pub fn BlogPost(props:&BlogProps) -> Html {
  let post_data: UseStateHandle<Option<Post>> = use_state(|| None);

  {
    let post_data = post_data.clone();
    let id = props.id.clone();
    use_effect_with((), move |_|{
      wasm_bindgen_futures::spawn_local(async move {
        let url = "https://jsonplaceholder.typicode.com/posts/".to_owned() + id.as_str();
        let data = get_data::<Post>(url.as_str())
        .await;

      match data {
        Ok(data)=>post_data.set(Some(data)),
        Err(err)=>{log!(format!("Error occured: {}", err))}
      };
        // post_data.set(Some(data));
      })
      
    })

  }

  return html! {
    <div>{format!("blog post id {}", props.id)}
      if !post_data.is_none() {
        <div>{&post_data.as_ref().unwrap().title}</div>
      }
    </div>
  }
}