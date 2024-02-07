use yew::prelude::*;

#[function_component]
pub fn Contact()->Html {

  html!{
    <div class="p-2">
      <div class="text-5xl mt-8 inline-block border-b-2 border-gray-500 py-2">
        {"Contact Me"}
      </div>
      <div class="grid grid-cols-2 gap-4">
        <div>
          <div class="text-2xl my-4">{"Get In Touch"}</div>
          <div>
            <div class="my-2">
              <label for="nameInput">{"Name"}</label>
              <input id="nameInput" placeholder="Enter Your Name" type="text" class="border border-gray-500 rounded-md p-2 w-full"/>
            </div>
            <div class="my-2">
              <label for="emailInput">{"Email"}</label>
              <input id="emailInput" placeholder="Enter Your Email" type="text" class="border border-gray-500 rounded-md p-2 w-full"/>
            </div>
            <div class="my-2">
              <label for="subjectInput">{"Subject"}</label>
              <input id="subjectInput" placeholder="Enter Your Subject" type="text" class="border border-gray-500 rounded-md p-2 w-full"/>
            </div>
            <div class="my-2">
              <label for="subjectInput">{"Message"}</label>
              <textarea id="subjectInput" placeholder="Enter Your Message" class="border border-gray-500 rounded-md p-2 w-full"/>
            </div>
          </div>
          <button class="cus_btn">{"send mail"}</button>
        </div>
        <div>
          <div class="bg-gray-200 rounded p-4 my-4 flex gap-4" >
            <div class="p-4 border border-gray-300 rounded">
              <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-12 h-12">
                <path stroke-linecap="round" stroke-linejoin="round" d="M2.25 6.75c0 8.284 6.716 15 15 15h2.25a2.25 2.25 0 0 0 2.25-2.25v-1.372c0-.516-.351-.966-.852-1.091l-4.423-1.106c-.44-.11-.902.055-1.173.417l-.97 1.293c-.282.376-.769.542-1.21.38a12.035 12.035 0 0 1-7.143-7.143c-.162-.441.004-.928.38-1.21l1.293-.97c.363-.271.527-.734.417-1.173L6.963 3.102a1.125 1.125 0 0 0-1.091-.852H4.5A2.25 2.25 0 0 0 2.25 4.5v2.25Z" />
              </svg>
            </div>
            <div> 
              <div>{"Phone"}</div>
              <div>{"+912376283732"}</div>
            </div>
          </div>
          <div class="bg-gray-200 rounded p-4 my-4 flex gap-4">
            <div class="p-4 border border-gray-300 rounded">
              <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-10 h-10">
                <path stroke-linecap="round" stroke-linejoin="round" d="M21.75 6.75v10.5a2.25 2.25 0 0 1-2.25 2.25h-15a2.25 2.25 0 0 1-2.25-2.25V6.75m19.5 0A2.25 2.25 0 0 0 19.5 4.5h-15a2.25 2.25 0 0 0-2.25 2.25m19.5 0v.243a2.25 2.25 0 0 1-1.07 1.916l-7.5 4.615a2.25 2.25 0 0 1-2.36 0L3.32 8.91a2.25 2.25 0 0 1-1.07-1.916V6.75" />
              </svg>
            </div>
            <div> 
              <div>{"Email"}</div>
              <div>{"abc@exmaple.com"}</div>
            </div>
        </div>
        </div>
      </div>
    </div>
  }
}