use leptos::*;
use leptos_meta::*;
use leptos_router::*;

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <Title text="Início"/>
        <div class="min-h-screen flex bg-no-repeat bg-[url('/assets/front-page.png')] bg-[length:100%]"></div>
        <A href="/login">"Fazer Login"</A>
    }
}
