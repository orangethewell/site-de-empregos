use gloo::console::log;
use yew::prelude::*;
use yew_icons::{Icon, IconId};
use yew_router::prelude::*;
use bounce::{Atom, use_atom};
use crate::app::Route;

#[derive(PartialEq, Debug, Clone)]
struct MenuRelativeIcon<'a> {
    title: &'a str,
    icon_id: IconId,
    refer_to: Route,
    gap: bool
}

#[derive(Atom, Default, PartialEq)]
pub struct ShowMenu {
    pub value: bool,
}

#[function_component(SidebarMenu)]
pub fn sidebar_menu() -> Html {
    let is_open = use_state(|| true);
    let current_route = use_route().unwrap_or(Route::Dashboard);
    let menus = use_state(|| vec![
        MenuRelativeIcon {title: "Painel de Controle", icon_id: IconId::LucideLayoutDashboard, refer_to: Route::Dashboard, gap: false},
        MenuRelativeIcon {title: "Vagas", icon_id: IconId::BootstrapBuildingExclamation, refer_to: Route::Jobs, gap: true},
        MenuRelativeIcon {title: "Configurações", icon_id: IconId::LucideSettings, refer_to: Route::Settings, gap: true}
    ]);
    let selected_index = use_state(|| {
        if let Some(index) = (*menus).iter().position(|menu| menu.refer_to == current_route) {
        index
        } else {0}
    });

    let menu_item_click = {
        let selected_index = selected_index.clone();
        let navigator = use_navigator().unwrap();
        Callback::from(move |(index, route)| {
            log!(format!("value: {}", index));
            navigator.push(&route);
            selected_index.set(index);
        })
    };

    let set_open = {
        let is_open = is_open.clone();
        Callback::from(move |_| is_open.set(!*is_open))
    };

    let is_menu_enabled = use_atom::<ShowMenu>();

    if is_menu_enabled.value {
        html! {
        <>
            <nav class={classes!(if *is_open {"w-72"} else {"w-20"}, "duration-300", "h-screen", "relative", "bg-red-900", "p-5", "pt-8")}>
                <img onclick={set_open} src="/admin/assets/control.png" class={classes!(if *is_open {"rotate-0"} else {"rotate-180"}, "absolute", "duration-500", "cursor-pointer", "-right-3", "top-9", "w-8", "border-2", "rounded-full", "border-red-900")}/>
                <ul class="pt-6">
                    {(*menus).clone().into_iter().enumerate().map(|(index, menu)| html! {
                        <li key={index} onclick={menu_item_click.clone().reform(move |_| (index, menu.refer_to.clone()))} class={classes!("text-gray-300", "text-sm", "flex", "items-center", "gap-x-4", "cursor-pointer", "p-2", "hover:bg-red-800", "rounded-md", if index == *selected_index {"bg-red-800"} else {""}, if menu.gap {"mt-9"} else {"mt-2"})}>
                            <Icon icon_id={menu.icon_id} class="flex-shrink-0"/>
                            <span class={classes!(if !*is_open {"opacity-0"} else {""}, "transition", "origin-left", "duration-200", "text-clip", "overflow-hidden", "whitespace-nowrap")}>{menu.title}</span>
                        </li>
                    }).collect::<Html>()}
                </ul>
            </nav>
        </>
        }
    } else {
        html! {}
    }
}