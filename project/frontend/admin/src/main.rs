pub mod app;

pub mod components;
pub mod pages;

use app::App;

// Entry point for starting the Yew application
pub fn main() {
    //Start the Yew framework
    yew::Renderer::<App>::new().render();
}
