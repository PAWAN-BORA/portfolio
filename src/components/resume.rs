use yew::prelude::*;


struct Skill {
  name: String,
  efficent:u8,
}
#[function_component]
pub fn Resume()->Html{
  let skills:Vec<Skill> = vec![
    Skill{name:"Rust".to_string(),efficent:75},
    Skill{name:"HTML".to_string(),efficent:95},
    Skill{name:"CSS".to_string(),efficent:92},
    Skill{name:"JavaScript".to_string(),efficent:95},
    Skill{name:"Yew".to_string(),efficent:75},
    Skill{name:"React".to_string(),efficent:95},
  ];
  


  html!{
    <div class="p-2">
      <div class="text-5xl mt-8 inline-block border-b-2 border-gray-500 py-2">
        {"My Skills"}
      </div>
      <div class="grid grid-cols-2 gap-4 mt-4">
        {for skills.iter().map(|skill| html!{
          <div class="text-2xl">
            <div>
              {&skill.name}
            </div>
            <div class="text-base flex gap-4 items-center font-light">
              <div>{skill.efficent}{"%"}</div>
              <div class="w-[100%] bg-gray-200">
                <div 
                  class={classes!(
                  "h-1.5",
                  "bg-blue-600",
                  "mr-4"
                  )}
                  style={format!("width:{}%",skill.efficent)}
                ></div>
              </div>
            </div>
          </div>
        })}
       
      </div>
    </div>
  }
}