use gloo::console::log;

use serde::{Deserialize, Serialize};
use yew::prelude::*;
use crate::components::services::services::get_data;
use crate::components::common::loader::Loader;
#[derive(Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
struct Post {
  user_id:i32,
  id:i32,
  title:String,
  body:String,
}

#[function_component]
pub fn Blog()->Html{
  let is_loading = use_state(|| true);
  let modal = use_state(|| false);
  let posts: UseStateHandle<Vec<Post>> = use_state(|| vec![]);
  let selected_post: UseStateHandle<Option<Post>> = use_state(||None);
  {
    let posts = posts.clone();
    let is_loading = is_loading.clone();
    use_effect_with((), move |_|{
      wasm_bindgen_futures::spawn_local(async move {
        let fetched_videos = get_data("https://jsonplaceholder.typicode.com/posts")
          .await;
        match fetched_videos {
            Ok(data) => {posts.set(data); is_loading.set(false)},
            Err(e) => {log!(format!("Error occured {}" , e));}
        };
          // Request::get("https://jsonplaceholder.typicode.com/posts")
          // .send()
          // .await
          // .unwrap()
          // .json()
          // .await
          // .unwrap();
        

        // let desrilize = serde_json::to_string_pretty(&fetched_videos).unwrap();
        // log!(desrilize);
      })
    })
  }
  let set_data = Callback::from({
    let modal = modal.clone();
    let selected_post = selected_post.clone();
    move |post:Post| {
      log!(serde_json::to_string(&post).unwrap());
      
      selected_post.set(Some(post));
      modal.set(true);
    }
  });

  let toggle_modal = Callback::from({
    let modal = modal.clone();
    move |val:bool| {
      modal.set(val);
    }
  });
  // fn set_data(data:Post, mut selectedPost:UseStateHandle<Option<Post>>){
  //   log!(serde_json::to_string_pretty(&data).unwrap());
   
  //   selectedPost.set(Some(data));
  // };
  // let handle_click = Callback::from(move |item_data: String| {
  //   println!("Clicked on item with ID: {} and name: ", item_data);
  // });
  if *is_loading {
    return html!{
      <div class="flex justify-center h-screen items-center">
        <Loader/>
      </div>
    };
  };
  html!{
    <div class="p-2 h-screen overflow-auto">
      {for posts.iter().map(|item|{
        let item = item.clone();
        let set_data = set_data.clone();
        html!{
          
          <div class="border-b border-gray-400 p-2">
            <div class="font-bold capitalize">{&item.title}</div>
            <div>{&item.body[0..20]}</div>
            <button class="cus_btn" 
              // onclick={Callback::from(move |_| {set_data.emit(item.clone())})}
              onclick={set_data.clone().reform(move |_| {item.clone()})}

            >
              {"Read More"}
            </button>
          </div>
        }
      })}
      if *modal {
        <div class="modal">
          <div class="modal-content">
            <div class="flex justify-between py-2 border-b border-gray-400">
              <div>
                {"Modal"}
              </div>
              <div onclick={toggle_modal.clone().reform(|_| false)} class="cursor-pointer">
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                  <path stroke-linecap="round" stroke-linejoin="round" d="m9.75 9.75 4.5 4.5m0-4.5-4.5 4.5M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z" />
                </svg>
              </div>
            </div>
            <div>
            if !selected_post.is_none() {
              <div class="font-bold uppercase my-2">{&selected_post.as_ref().unwrap().title}</div>
              <div>{&selected_post.as_ref().unwrap().body}</div>
            } 
            </div>
          </div>
        </div>
      }
      
    </div>
  }
}