pub mod pages;
pub mod app;
pub mod components;

use app::App;

// Entry point for starting the Yew application
pub fn main() {
    //Start the Yew framework
    yew::Renderer::<App>::new().render();
}
