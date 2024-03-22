use leptos::*;
use leptos_router::*;
use leptos_meta::*;

use crate::functions::confirm_email_changes;

#[derive(Params, PartialEq)]
struct MailConfirmationSearch {
    code: String,
}

#[component]
pub fn MailConfirmation() -> impl IntoView {
    let query = use_query::<MailConfirmationSearch>();

    let confirm_mail = move |_| {
        let code = move || query.with(|query| {
            query.as_ref()
                .map(|query| query.code.clone())
                .unwrap_or_default()
        });

        spawn_local(async move {
            confirm_email_changes(code()).await.unwrap()
        });
    };

    view! {
        <Title text="Confirmar E-Mail"/>
        <main class="flex justify-center w-full bg-slate-200 min-h-screen">
            <div class="mx-2 md:w-1/2 my-4 w-full self-start bg-white shadow-md">
                <h1 class="text-2xl my-2 font-bold">"Confirme seu E-mail"</h1>
                <hr class="mx-4"/>
                <p>"Olá! Ao clicar no botão abaixo, você concorda que tem acesso a esse e-mail e que poderá receber atualizações e informações do website através dele."</p>
                <button
                    on:click=confirm_mail
                    class="rounded-md hover:shadow-[0_2px_8px_0_rgba(0,0,0,0.5)] transition-all bg-[#ef4444] hover:bg-[#d83c3c] text-white p-2 m-5"
                >
                    "Confirmar E-mail"
                </button>
            </div>
        </main>
    }
}
