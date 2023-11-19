use yew::prelude::*;
use yew_router::prelude::*;
use yew_icons::{Icon, IconId};
use crate::app::Route;

#[function_component(FooterShield)]
fn nav_bar_shield() -> Html {
    html! {
        <footer class="bg-slate-900 text-white">
            <div class="flex items-center justify-center">
                <Link<Route> to={Route::Home}>{"Início"}</Link<Route>>
                <Link<Route> to={Route::Jobs}>{"Vagas"}</Link<Route>>
            </div>
            <div class="flex items-center justify-center">
            
            </div>
        </footer>
    }
}

#[function_component(Footer)]
pub fn footer() -> Html {
    // Don't show navbar on these routes
    let service: Route = use_route().unwrap_or_default();
    let denied_routes = vec![];

    html! {
        if !denied_routes.contains(&service) {
          <FooterShield/>
        }
    }
}