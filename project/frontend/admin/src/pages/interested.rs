use bounce::use_atom;
use yew::prelude::*;

use crate::components::menu::ShowMenu;

#[function_component(Interested)]
pub fn interested() -> Html {
    let is_menu_enabled = use_atom::<ShowMenu>();
    is_menu_enabled.set(ShowMenu {value: true});

    html! {
        <div>
            <h1>{"Interessados"}</h1>
        </div>
    }
}