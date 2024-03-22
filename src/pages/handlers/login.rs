use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::{components::{Footer, Navbar}, functions::{get_logged_user, LoginUser, UserModel}};

/// Renders the home page of your application.
#[component]
pub fn Login() -> impl IntoView {
    let log_in = create_server_action::<LoginUser>();

    let message = create_rw_signal(String::new());

    let value = log_in.value();
    let error_message = move || {
        value.with(
            |val| {
                if let Some(Err(message)) = val {
                    message.to_string()
                } else {
                    String::new()
                }
            }
        )
    };

    let on_submit = move |_| {
        message.set(error_message())
    };
    
    let user = create_resource(move || (), |_| async move {get_logged_user().await});

    view! {
        <Title text="login"/>
        <Transition fallback=move || {
            view! { <h1>"Carregando..."</h1> }
        }>
            {move || {
                user.get()
                    .map(|user| match user {
                        Ok(user) => {
                            view! {
                                <>
                                    <h1>
                                        "Login já realizado, redirecionando para página do usuário \""
                                        {user.username} "\""
                                    </h1>
                                    <Redirect path="/perfil"/>
                                </>
                            }
                        }
                        Err(_) => {
                            view! {
                                <>
                                    {
                                        view! {}
                                    }
                                </>
                            }
                        }
                    })
            }}

        </Transition>
        <main class="flex justify-center w-full bg-slate-200 min-h-screen">
            <div class="mx-2 md:w-1/2 my-4 w-full self-start bg-white shadow-md">
                {move || {
                    if !message().is_empty() {
                        view! {
                            <>
                                <p class="border m-4 border-red-500 bg-red-200 text-red-500">
                                    {message()}
                                </p>
                            </>
                        }
                    } else {
                        view! {
                            <>
                                {
                                    view! {}
                                }
                            </>
                        }
                    }
                }}
                <ActionForm on:submit=on_submit action=log_in class="p-4">
                    <h1 class="text-3xl font-bold mb-2">"Login"</h1>
                    <hr class="mx-4 mb-6 h-[2px] bg-gray-300"/>
                    <div class="mb-4">
                        <p class="text-left font-bold">"Email"</p>
                        <input
                            id="email"
                            name="email"
                            class="rounded-md border-2 w-full border-gray-400 flex-grow p-2 focus:outline-none focus:border-gray-600"
                            type="email"
                        />
                    </div>
                    <div class="mb-4">
                        <p class="text-left font-bold">"Senha"</p>
                        <input
                            id="password"
                            name="password"
                            class="rounded-md border-2 w-full border-gray-400 flex-grow p-2 focus:outline-none focus:border-gray-600"
                            type="password"
                        />
                    </div>
                    <input
                        type="submit"
                        class="rounded-md hover:shadow-[0_2px_8px_0_rgba(0,0,0,0.5)] transition-all bg-[#ef4444] hover:bg-[#d83c3c] text-white p-2 w-full mb-5"
                        value="Login"
                    />
                    <p>
                        "Ainda não fez sua inscrição? "
                        <A class="text-blue-600" href="/cadastrar">
                            "Se inscreva aqui!"
                        </A>
                    </p>
                    <p>
                        "Esqueceu sua senha de acesso? "
                        <A class="text-blue-600" href="/login/recuperar-senha">
                            "Clique aqui."
                        </A>
                    </p>
                </ActionForm>
            </div>
        </main>
    }
}