
mod app;
mod components;
use app::App;
// mod components;
fn main() {
    yew::Renderer::<App>::new().render();
}
