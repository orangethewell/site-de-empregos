use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::functions::is_user_logged_in;
// Handler Pages
use crate::pages::handlers::{
    NotFound,
    Register,
    Login
};

// Global Components
use crate::components::{
    Navbar,
    Footer
};

// Pages
use crate::pages::{
    HomePage,
    JobsPage,
    Profile
};

#[derive(Copy, Clone)]
pub struct NavbarReloader(pub WriteSignal<bool>);

#[component]
pub fn NavbarUpdater() -> impl IntoView {
    let set_reload = use_context::<NavbarReloader>().unwrap().0;
    set_reload.update(|toggle: &mut bool| { *toggle = !*toggle; });
    view! {

    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    let (reload_navbar, set_reload_navbar) = create_signal(false);

    let is_logged = create_resource(reload_navbar, |_| async move { is_user_logged_in().await.unwrap() });

    provide_context(NavbarReloader(set_reload_navbar));
    provide_meta_context();

    let formatter = |text| format!("{text} — Vagas em Araxá");

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title formatter/>
        // content for this welcome page
        <Router>
            <Navbar is_logged/>
            <div>
                <Routes>
                    <Route path="/" view=move || {
                        view! {
                            <NavbarUpdater/>
                            <HomePage/>
                        }}/>
                    <Route path="/vagas" view=move || {
                        view! {
                            <NavbarUpdater/>
                            <Redirect path="/vagas/1"/>
                        }
                    }/>
                    <Route path="/vagas/:page_id" view=move || {
                        view! {
                            <NavbarUpdater/>
                            <JobsPage/>
                        }}/>
                    <Route path="/login" view=move || {
                        view! {
                            <NavbarUpdater/>
                            <Login/>
                        }}/>
                    <Route path="/cadastrar" view=move || {
                        view! {
                            <NavbarUpdater/>
                            <Register/>
                        }}/>
                    <Route path="/meu-perfil" view=move || {
                        view! {
                            <NavbarUpdater/>
                            <Profile/>
                        }}/>
                    <Route path="/*any" view=move || {
                        view! {
                            <NavbarUpdater/>
                            <NotFound/>
                        }}/>
                </Routes>
            </div>
                <Footer/>
        </Router>
        
    }
}

