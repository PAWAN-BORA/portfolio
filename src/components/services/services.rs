// use wasm_bindgen_futures

use gloo_net::http::{Request};
use gloo_net::Error;
use serde::Deserialize;

pub async fn get_data<T:Clone+PartialEq+for<'a> Deserialize<'a>>(url: &str)->Result<T, Error> {

  let data:T = Request::get(url)
  .send()
  .await?
  .json()
  .await?;
  return  Ok(data);
}
