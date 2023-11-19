use yew::prelude::*;
use yew_router::prelude::*;
use yew_icons::{Icon, IconId};
use crate::app::Route;

#[derive(PartialEq, Properties)]
struct NavButtonProps {
  #[prop_or_default]
  class: Classes,
  children: Children,
  to: Route,
  #[prop_or_default]
  is_mobile: bool
}

#[function_component(NavButton)]
fn nav_button(props: &NavButtonProps) -> Html {
  let mut classes = classes!({if props.is_mobile {
    vec!["block", "py-4", "px-4", "text-sm", "transition", "easy-in-out", "hover:bg-red-300"]
  } else {
    vec!["py-2", "px-3", "transition", "easy-in-out", "hover:bg-red-300"]
  }});

  classes.push(props.class.clone());

  html! {
    <Link<Route> to={props.to.clone()} {classes}>{props.children.clone()}</Link<Route>>
  }
}

#[function_component(NavBarShield)]
fn nav_bar_shield() -> Html {
  let is_open = use_state(|| false);
  let onclick = {
    let is_open = is_open.clone();
    Callback::from(move |_| {
      if *is_open == true {
        is_open.set(false)
      } else {
        is_open.set(true)
      }
    })
  };
  html! {
    <nav class="bg-[#ef4444] text-white">
      <div class="xl:max-w-6xl xl:px-4 mx-auto">
      <div class="flex items-center justify-center"><img class="object-contain h-40 w-40" src="assets/logo_white.png"/></div>
        <div class="flex justify-center">
          <div class="flex space-x-4">
            <div class="hidden md:flex items-center space-x-1">
              <NavButton to={Route::Home}>{"Início"}</NavButton>
              <NavButton to={Route::Jobs}>{"Vagas"}</NavButton>
            </div>
          </div>
          // <div class="hidden md:flex items-center space-x-1">
          //   <NavButton class="border-l-[1px]" to={Route::Home}>{"Registrar"}</NavButton>
          //   <NavButton to={Route::Login}>{"Login"}</NavButton>
          // </div>

          // Menu on Mobile
          <div class="md:hidden flex items-center space-x-1">
            <button data-te-ripple-init="true" data-te-ripple-color="#fca5a5" class="py-2 px-3" {onclick}>{"Menu"}</button>
          </div>
        </div>
      </div>
      // Another Menu
      if *is_open {
        <div class="md:hidden transition-all">
          <NavButton to={Route::Home} is_mobile=true>{"Início"}</NavButton>
          <NavButton to={Route::Jobs} is_mobile=true>{"Vagas"}</NavButton>
          // <NavButton class="border-t-[1px]" to={Route::Home} is_mobile=true>{"Registrar"}</NavButton>
          // <NavButton to={Route::Login} is_mobile=true>{"Login"}</NavButton>
        </div>
      }
    </nav>
  }
}

#[function_component(Navbar)]
pub fn nav_bar() -> Html {
    // Don't show navbar on these routes
    let service: Route = use_route().unwrap_or_default();
    let denied_routes = vec![];

    html! {
        if !denied_routes.contains(&service) {
          <NavBarShield/>
        }
    }
}