use yew::prelude::*;

#[derive(PartialEq, Clone, Properties)]
struct CardData {
  icon:Html,
  title:String,
  body:String,
}



#[function_component]
pub fn About()->Html{
 
  let services:Vec<CardData> = vec![
    CardData {icon:html!{<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
              <path stroke-linecap="round" stroke-linejoin="round" d="M9.53 16.122a3 3 0 0 0-5.78 1.128 2.25 2.25 0 0 1-2.4 2.245 4.5 4.5 0 0 0 8.4-2.245c0-.399-.078-.78-.22-1.128Zm0 0a15.998 15.998 0 0 0 3.388-1.62m-5.043-.025a15.994 15.994 0 0 1 1.622-3.395m3.42 3.42a15.995 15.995 0 0 0 4.764-4.648l3.876-5.814a1.151 1.151 0 0 0-1.597-1.597L14.146 6.32a15.996 15.996 0 0 0-4.649 4.763m3.42 3.42a6.776 6.776 0 0 0-3.42-3.42" />
            </svg>},
      title:String::from("Web Design"),
      body:String::from("I can provide clean code and pixel perfect design. I also make website more & more interactive with web animations.")

    },
    CardData {
      icon: html!(<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
              <path stroke-linecap="round" stroke-linejoin="round" d="M17.25 6.75 22.5 12l-5.25 5.25m-10.5 0L1.5 12l5.25-5.25m7.5-3-4.5 16.5" />
            </svg>
            ),
      title:String::from("Web Development"),
      body:String::from("I can provide clean code and pixel perfect design. I also make website more & more interactive with web animations.")
    },
    CardData {
      icon: html!(<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
            <path stroke-linecap="round" stroke-linejoin="round" d="M10.5 1.5H8.25A2.25 2.25 0 0 0 6 3.75v16.5a2.25 2.25 0 0 0 2.25 2.25h7.5A2.25 2.25 0 0 0 18 20.25V3.75a2.25 2.25 0 0 0-2.25-2.25H13.5m-3 0V3h3V1.5m-3 0h3m-3 18.75h3" />
          </svg>
          ),
      title:String::from("Mobile Application"),
      body:String::from("I can provide clean code and pixel perfect design. I also make website more & more interactive with web animations.")
    },
  ];

  html!{
    <div class="about p-2 h-screen overflow-auto">
      <div class="my-10 text-3xl uppercase border-b pb-2 inline-block">
        {"About Me"}
      </div>
      <div class="grid grid-cols-[500px_1fr] gap-4 "> 
        <div> 
          <img src="./assets/images/logo.jpg" class="w-full rounded"/>
        </div>
        <div>
          <div class="text-4xl">{"Hi, I am"}
            <span class="text-blue-700">{"  Pawan"}</span>
          </div>
          <div class="mt-4 font-light">
            {"I am a frontend web developer. I can provide clean code and pixel perfect design. I also make website more & more interactive with web animations."}
          </div>
          <div class="grid grid-cols-[150px_1fr] gap-2 mt-8">
            <div class="font-bold">{"Full Name"}</div>
            <div class="font-light">{": Pawan Singh Bora"}</div>
            <div class="font-bold">{"Nationality"}</div>
            <div class="font-light">{": Indian"}</div>
            <div class="font-bold">{"Languages"}</div>
            <div class="font-light">{": English, Hindi"}</div>
          </div>
          <div class="mt-12">
            <button class="cus_btn">{"Download CV"}</button>
          </div>
        </div>
      </div>
      <div class="my-10 text-3xl  uppercase border-b pb-2 inline-block">
        {"Services"}
      </div>
      <div class="grid grid-cols-3 gap-2" >
        {for services.iter().map(|item| html!{ <Card data={item.clone()}/>})}
      </div>
    </div>
  }
}

#[derive(PartialEq, Clone, Properties)]
struct CardProps{
  data:CardData
}
#[function_component]
fn Card(props: &CardProps)->Html{

  html!{
    <div class="border p-4">
      <div>
        {props.data.icon.clone()}
        <div class="text-2xl border-b-2 inline-block pb-2 border-gray-400">
          {&props.data.title}
        </div>
        <div class="mt-4">
          {"Lorem ipsum dolor sit amet consectetur adipisicing elit. Autem tenetur ratione quod."}
        </div>
      </div>
    </div>
  }
}