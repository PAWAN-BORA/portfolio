use yew::prelude::*;

#[function_component]
pub fn Home()->Html{

  return html! {
    <div class="h-screen flex items-center justify-center ">
      <div class="max-w-screen-sm text-center">
        // <div class="text-4xl">
          <div class="text-center text-4xl">{"Hi, I am"}
            <span class="text-blue-700">{"  Pawan"}</span>
          </div>
          <div class="mt-4">
            {"I am a frontend web developer. I can provide clean code and pixel perfect design. I also make website more & more interactive with web animations."}
          </div>
        // </div>
      </div>
    </div>
  };
}