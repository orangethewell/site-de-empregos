use leptos::*;
use leptos_meta::*;
use leptos_router::*;

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <Title text="Início"/>
        <h1>"Vagas em araxá 2"</h1>
        <p>"Hello, world!"</p>
        <A href="/login">"Fazer Login"</A>
    }
}