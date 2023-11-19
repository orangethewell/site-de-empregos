use bounce::use_atom;
use yew::prelude::*;

use crate::components::menu::ShowMenu;

#[function_component(Settings)]
pub fn settings() -> Html {
    let is_menu_enabled = use_atom::<ShowMenu>();
    is_menu_enabled.set(ShowMenu {value: true});
    
    html! {
        <div>
            <h1>{"Settings"}</h1>
        </div>
    }
}