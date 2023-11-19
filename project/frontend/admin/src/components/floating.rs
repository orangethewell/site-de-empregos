use gloo::console::log;
use yew::prelude::*;
use yew_icons::{Icon, IconId};
use yew_router::prelude::*;
use crate::app::Route;

#[derive(Properties, PartialEq)]
pub struct FloatWindowProps {
    #[prop_or_default]
    pub children: Html,
    pub onclose: Callback<MouseEvent>
}

#[function_component(FloatWindow)]
pub fn floating_window(props: &FloatWindowProps) -> Html {
    html! {
        <div class="bg-gray-700/20 h-screen fixed w-screen inset-0 z-10 backdrop-blur-[3px] flex justify-center items-center">
            <div class="bg-white max-w-lg shadow-lg fixed rounded-lg z-20">
                <div class="bg-red-500 h-8 rounded-t-lg">
                    <button onclick={props.onclose.clone()} class="float-right p-1 px-2 text-white rounded-tr-lg transition hover:bg-red-600">{"X"}</button>
                </div>
                <div class="p-4 max-h-96 overflow-y-auto">
                    {props.children.clone()}
                </div>
            </div>
        </div>
    }
}