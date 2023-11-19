use std::collections::HashMap;

use common::get_user_info;
use gloo::console::log;
use yew::platform::spawn_local;
use yew::prelude::*;
use yew_router::{prelude::*, navigator};
use yew_notifications::{NotificationsProvider, NotificationsPosition};
use bounce::{BounceRoot, use_atom};
use crate::components::menu::ShowMenu;
use crate::components::{SidebarMenu, AuthGuard, AuthProtected, AuthFallback};
use crate::pages::*;

use crate::components::notifications::factory::CustomNotificationFactory;
use crate::components::notifications::CustomNotification;
use crate::pages::login::LoggedUser;

#[derive(Clone, Routable, PartialEq, Debug, serde::Serialize)]
pub enum Route {
    #[at("/")]
    Dashboard,

    #[at("/interessados")]
    Interested,

    #[at("/vagas")]
    Jobs,

    #[at("/configuracoes")]
    Settings,

    #[at("/login")]
    Login,
}

pub fn switch(routes: Route) -> Html {
    let loading_unit = html! {
        <div class="flex h-full items-center justify-center">
            <div
            class="inline-block h-16 w-16 animate-spin rounded-full border-4 border-solid border-current border-r-transparent align-[-0.125em] motion-reduce:animate-[spin_1.5s_linear_infinite]"
            role="status">
            <span
                class="!absolute !-m-px !h-px !w-px !overflow-hidden !whitespace-nowrap !border-0 !p-0 ![clip:rect(0,0,0,0)]"
                >{"Loading..."}
            </span>
            </div> 
        </div>
    };
    match routes {
        Route::Dashboard => {
            html! {
            <AuthGuard loading_unit={loading_unit.clone()} permission="DashboardViewer">
                <AuthProtected>
                    <Dashboard/>
                </AuthProtected>
                <AuthFallback>
                    <Restrict/>
                </AuthFallback>
            </AuthGuard>
            }
        }
        Route::Interested => {
            html! {
            <AuthGuard loading_unit={loading_unit.clone()} permission="DashboardViewer">
                <AuthProtected>
                    <Interested/>
                </AuthProtected>
                <AuthFallback>
                    <Restrict/>
                </AuthFallback>
            </AuthGuard>
            }
        }
        Route::Jobs => {
            html! {
            <AuthGuard loading_unit={loading_unit.clone()} permission="DashboardViewer">
                <AuthProtected>
                    <Jobs/>
                </AuthProtected>
                <AuthFallback>
                    <Restrict/>
                </AuthFallback>
            </AuthGuard>
            }
        }
        Route::Settings => {
            html! {
            <AuthGuard loading_unit={loading_unit.clone()} permission="DashboardViewer">
                <AuthProtected>
                    <Settings/>
                </AuthProtected>
                <AuthFallback>
                    <Restrict/>
                </AuthFallback>
            </AuthGuard>
            }
        }

        Route::Login => {
            html! {
            <Login/>
            }
        }
    }
}

#[function_component(Restrict)]
pub fn restrict() -> Html {
    let is_menu_enabled = use_atom::<ShowMenu>();
    is_menu_enabled.set(ShowMenu {value: false});

    html! {
        <div class="h-screen fixed w-screen flex justify-center items-center">
            <div class="max-w-2xl">
                <h1 class="text-8xl mb-12 font-bold flex items-end"><span class="flex-grow">{"Ops! "}</span><span class="text-4xl">{"Acesso Restrito"}</span></h1>
                <p class="text-xl mb-8 p-4 text-center border-[3px] border-gray-200">{"Parece que você entrou em um site restrito a usuários privilegiados,
                e você não tem o privilégio de acesso. Se acha que isso é um engano,
                converse com um usuário administrador para resolver esse problema."}</p>
                <a class="text-gray-400 hover:underline flex justify-center" href="http://www.vagasemaraxa.com:5000/">{"voltar ao site principal"}</a>
            </div>
        </div>
        
    }
}

#[function_component(AdminPanel)]
pub fn admin_panel() -> Html {
    let logged_user = use_atom::<LoggedUser>();
    let navigator = use_navigator().unwrap();

    {
        let logged_user = logged_user.clone();
        let navigator = navigator.clone();
        use_effect_with((), move |_| {
            let logged_user = logged_user.clone();
            let navigator = navigator.clone();
            if logged_user.id == "" {
                spawn_local(async move {
                    match get_user_info().await {
                        Ok(user) => logged_user.set(LoggedUser { id: user.id, username: user.username, email: user.email }),
                        Err(_) => {log!("error 1"); navigator.push(&Route::Login)}
                    }
                })
            }
        })
    }

    html! {
        <div class="flex">
            <SidebarMenu />
            <main class="flex-1 p-7 h-screen bg-gray-50">
                <Switch<Route> render={switch}/>
            </main>
        </div>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let component_creator = CustomNotificationFactory::default();
    let position = NotificationsPosition::Custom("fixed z-50 space-y-4 right-8 bottom-8".into());

    html! {
        <BounceRoot>
        <BrowserRouter>
            <NotificationsProvider<CustomNotification, CustomNotificationFactory> {component_creator} {position}>
                <AdminPanel/>  
            </NotificationsProvider<CustomNotification, CustomNotificationFactory>>
            <script src="https://cdn.jsdelivr.net/npm/tw-elements/dist/js/tw-elements.umd.min.js"></script>
        </BrowserRouter>
        </BounceRoot>
    }
}
