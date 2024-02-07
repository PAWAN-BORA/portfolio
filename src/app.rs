

use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::common::sidebar::Sidebar;
use crate::components::contact::Contact;
use crate::components::home::Home;
use crate::components::about::About;
use crate::components::blog::Blog;
use crate::components::blog_post::BlogPost;
use crate::components::resume::Resume;
use crate::components::portfolies::Portfolio;
#[derive(Clone, Routable, PartialEq)]
pub enum Route {
  #[at("/")]
  Home,
  #[at("/about")]
  About,
  #[at("/blog")]
  Blog,
  #[at("/blog/:id")]
  BlogPost{id: String},
  #[at("/Resume")]
  Resume,
  #[at("/portfolio")]
  Portfolio,
  #[at("/contact")]
  Contact,
  #[at("/*")]
  NotFound,
}


fn switch(routes: Route) -> Html {
  match routes {
      Route::Home => html! { <Home/>},
      Route::About => html! {<About/>},
      Route::Blog => html! {<Blog/>},
      Route::BlogPost { id }=>html!(<BlogPost id={id}/>),
      Route::Resume => html! {<Resume/>},
      Route::Portfolio=> html! {<Portfolio/>},
      Route::Contact=> html! {<Contact/>},
      Route::NotFound => html! { <h1>{ "404 for all" }</h1> },
  }
}

#[function_component]
pub fn App() -> Html {
    html! {
      <BrowserRouter>
        <div class="grid grid-cols-[250px_1fr]">
          <Sidebar />
          <Switch<Route> render={switch} />
        </div>
      </BrowserRouter>
    }
}