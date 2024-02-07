use std::cmp;

use gloo::console::log;
use serde::{Deserialize, Serialize};
use yew::prelude::*;
use crate::components::services::services::get_data;
// use crate::components::common::loader::Loader;
#[derive(PartialEq, Deserialize, Serialize, Clone)]
struct Product {
  id:u8,
  title:String,
  thumbnail:String,
  description:String,
}
#[derive(PartialEq, Serialize, Deserialize, Clone)]
struct ResType {
  products:Vec<Product>,
  total:u32,
  skip:u32,
  limit:u16,
}

#[function_component]
pub fn Portfolio()->Html{
  
  let products:UseStateHandle<Vec<Product>> = use_state(|| vec![]);
  let limit:u32 = 11;
  let skip: UseStateHandle<u32> = use_state(|| 0);
  let total: UseStateHandle<u32> =use_state(|| 0);
  {
    let products = products.clone();
    let total = total.clone();
    let skip = skip.clone();
    use_effect_with((), move |_|{
      wasm_bindgen_futures::spawn_local(async move {
        let fetched_products: Result<ResType, gloo_net::Error> = get_data(&format!("https://dummyjson.com/products?skip={}&limit={}", *skip, limit)).await;
        match fetched_products {
          Ok(data) => {products.set(data.products); total.set(data.total)},
          Err(err) => {log!(format!("Error occurec product {}", err))}
        }
      })
    });
  }
  let change_page = Callback::from({
    let products = products.clone();
    let total = total.clone();
    let skip = skip.clone();
    move |page_no:u32| {
      log!(page_no);
      let skip_item = page_no * limit;
      skip.set(skip_item);
      let products = products.clone();
      let total = total.clone();
      wasm_bindgen_futures::spawn_local(async move {
        let fetched_products: Result<ResType, gloo_net::Error> = get_data(&format!("https://dummyjson.com/products?skip={}&limit={}", skip_item, limit)).await;

        match fetched_products {
          Ok(data) => {products.set(data.products); total.set(data.total)},
          Err(err) => {log!(format!("Error occurec product {}", err))}
        }
      })
    }
  });
  // fn change_page(page_no:u32){
  //   log!(page_no);
  //   let products = products.clone();
  //   let total = total.clone();
  //   let skip = skip.clone();

  // }
  let page_no = (*skip / limit) + 1;
  let total_pages = (*total as f32/ limit as f32).ceil() as u32;
  return html!{
    <div class="p-2 h-screen overflow-y-auto">
      <div class="text-5xl mt-8 inline-block border-b-2 border-gray-500 py-2">
        {"Portfolio"}
      </div>
      <div class="grid grid-cols-3 gap-4 mt-8">
        {for products.iter().map(|product|{
          html!{
            <div class="">
              <div class="h-48">
                <img src={product.thumbnail.clone()} class="w-full h-full object-cover"/>
              </div>
              <div class="text-xl my-2">
                {&product.title}
              </div>
              <div clas="mb-2">
                {&product.description[..std::cmp::min(product.description.len(), 40)]}
              </div>
            </div>
          }
        })}
      </div>
      <div class="flex items-center justify-between border-t border-gray-200 bg-white px-4 py-3 sm:px-6 mt-4">
        <div class="flex flex-1 justify-between sm:hidden">
          <a class="relative inline-flex items-center rounded-md border border-gray-300 bg-white px-4 py-2 text-sm font-medium text-gray-700 hover:bg-gray-50">{"Previous"}</a>
          <a class="relative ml-3 inline-flex items-center rounded-md border border-gray-300 bg-white px-4 py-2 text-sm font-medium text-gray-700 hover:bg-gray-50">{"Next"}</a>
        </div>
        if products.len() > 0{
          <div class="hidden sm:flex sm:flex-1 sm:items-center sm:justify-between">
              <div>
                <p class="text-sm text-gray-700">
                  {"Showing "}
                  <span class="font-medium">{*skip+1}</span>
                  {" to "}
                  <span class="font-medium">{*skip+ cmp::min(products.len() as u32, limit)}</span>
                  {" of "}
                  <span class="font-medium">{*total}</span>
                  {" results"}
                </p>
              </div>
              <div>
              <nav class="isolate inline-flex -space-x-px rounded-md shadow-sm" aria-label="Pagination">
                <a
                  class={classes!(
                    "relative", "inline-flex", "items-center", "rounded-l-md", "px-2", "py-2", "text-gray-400", "ring-1", "ring-inset", "ring-gray-300", "hover:bg-gray-50", "focus:z-20", "focus:outline-offset-0",
                    if page_no>1 {"cursor-pointer"}else {""},
                  )}
                  
                  onclick={{
                    let change_page = change_page.clone(); 
                    move |_| {
                      if page_no > 1{
                        log!(page_no);
                        change_page.emit(page_no-2);
                      }
                      
                    }
                  }}>
                  <span class="sr-only">{"Previous"}</span>
                  <svg class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                    <path fill-rule="evenodd" d="M12.79 5.23a.75.75 0 01-.02 1.06L8.832 10l3.938 3.71a.75.75 0 11-1.04 1.08l-4.5-4.25a.75.75 0 010-1.08l4.5-4.25a.75.75 0 011.06.02z" clip-rule="evenodd" />
                  </svg>
                </a>
                {for (0..total_pages).map(|x|{
                    let common_classes = "relative inline-flex items-center font-semibold px-4 py-2 text-sm focus:z-20 cursor-pointer";
                    let mut classes = "text-gray-900 ring-1 ring-inset ring-gray-300 hover:bg-gray-50 focus:outline-offset-0";
                    if x+1==page_no{
                      classes = "z-10 bg-indigo-600 text-white focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600";
                    } 
                    html!(<a aria-current="page" class={classes!(
                      common_classes,
                      classes,
                    )} onclick={{let change_page = change_page.clone(); move |_| {change_page.emit(x);}}}>{x+1}</a>)
                  })
                }
                <a 
                 
                  class={classes!(
                    "relative", "inline-flex", "items-center", "rounded-r-md", "px-2", "py-2", "text-gray-400", "ring-1", "ring-inset", "ring-gray-300", "hover:bg-gray-50", "focus:z-20", "focus:outline-offset-0",
                    if page_no < total_pages {"cursor-pointer"}else {""},
                  )}
                  onclick={{
                    let change_page = change_page.clone(); 
                    move |_| {
                      if page_no < total_pages{
                        log!(page_no);
                        change_page.emit(page_no);
                      }
                    }
                  }}
                >
                  <span class="sr-only">{"Next"}</span>
                  <svg class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                    <path fill-rule="evenodd" d="M7.21 14.77a.75.75 0 01.02-1.06L11.168 10 7.23 6.29a.75.75 0 111.04-1.08l4.5 4.25a.75.75 0 010 1.08l-4.5 4.25a.75.75 0 01-1.06-.02z" clip-rule="evenodd" />
                  </svg>
                </a>
              </nav>
            </div>
          </div>
        }
      </div>
    </div>
  }
}