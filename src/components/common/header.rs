use yew::prelude::*;

#[function_component]
pub fn Header() -> Html {
  html! {
    <div class="test_clas">
      {"This is header"}
      <div class="bg-red-400" style="width: 100px"> 
        {"this is second"}
      </div>
    </div>
  }
}