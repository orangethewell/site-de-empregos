use leptos::{logging::log, *};
use leptos_meta::*;
use leptos_router::*;

use crate::functions::{generate_payment_url};

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    let on_click = move |_| {
        spawn_local(async move {
            let data = generate_payment_url().await.unwrap();
            log!("{}", data);
        })
    };

    view! {
        <Title text="Início"/>
        <div class="min-h-screen flex bg-no-repeat bg-[url('/assets/front-page.png')] bg-[length:100%]"></div>
        <button on:click=on_click>"Teste"</button>
        <A href="/login">"Fazer Login"</A>
    }
}