use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn UsersEditor() -> impl IntoView {
    view! {
        <Title text="Início"/>
        <h1>"Vagas em araxá 2"</h1>
        <p>"Hello, world!"</p>
        <A href="/login">"Fazer Login"</A>
    }
}