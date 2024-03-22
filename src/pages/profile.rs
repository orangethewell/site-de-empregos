use std::{collections::HashMap, time::Duration};

use leptos::{ev::MouseEvent, logging::log, *};
use leptos_meta::*;
use leptos_router::*;
use leptos_icons::*;
use icondata as i;

use crate::functions::{assign_role_to_user, change_email, change_username, edit_password, get_logged_user, get_membership_details, get_roles, get_user_roles, unsign_role_from_user, user_have_permission, UserModel};

const CLASS: &'static str = "p-16 text-right";

#[component]
pub fn UserInfo(user: UserModel) -> impl IntoView {
    let system_roles = create_resource(|| (), |_| async move {get_roles().await.unwrap()});
    let user_roles = create_resource(move || user.id, |id| async move {get_user_roles(id).await.unwrap()});
    let user_membership = create_resource(move || user.id, |id| async move {get_membership_details(id).await.unwrap()});

    let user_page_perms = create_resource(move || user.id, |id| async move {
        let mut user_perms = HashMap::new();
        user_perms.extend(
            vec![
                ("GerenciarCargos".to_owned(), user_have_permission(id, "GerenciarCargos".to_owned()).await.unwrap()),
                ("EditarVagas".to_owned(), user_have_permission(id, "EditarVagas".to_owned()).await.unwrap()),
                ("ModerarUsuários".to_owned(), user_have_permission(id, "ModerarUsuários".to_owned()).await.unwrap()),
                ("EditarUsuários".to_owned(), user_have_permission(id, "EditarUsuários".to_owned()).await.unwrap()),
                ("EditarArtigosPessoais".to_owned(), user_have_permission(id, "EditarArtigosPessoais".to_owned()).await.unwrap()),
                ("EditarDestaques".to_owned(), user_have_permission(id, "EditarDestaques".to_owned()).await.unwrap()),
                ("ConfigurarSite".to_owned(), user_have_permission(id, "ConfigurarSite".to_owned()).await.unwrap()),
                ("EditarAprovados".to_owned(), user_have_permission(id, "EditarAprovados".to_owned()).await.unwrap())
            ]
        );
        user_perms
    });

    let message = create_rw_signal(String::new());
    let username = create_rw_signal(user.username.clone());
    let username_backup = create_rw_signal(username.get_untracked());

    let email = create_rw_signal(user.email.clone());
    let email_backup = create_rw_signal(email.get_untracked());
    let email_confirmed = create_rw_signal(user.is_confirmed.clone());

    let password = create_rw_signal(String::new());
    let password_confirm = create_rw_signal(String::new());

    let is_valid_account = move || {
        let assertion = !email().ends_with("@localhost") && email_confirmed();
        if email().ends_with("@localhost") {
            message.set("Seu endereço de e-mail é inválido! Por favor, para manter a segurança da sua conta, mude seu endereço de e-mail.".to_owned());
        } else if !email_confirmed() {
            message.set("Confirme seu endereço de e-mail para liberar seu acesso ao sistema!".to_owned());
        }
        assertion
    };

    let role_menu_open = create_rw_signal(false);
    let is_editing = create_rw_signal("");

    let edit_field = move |field| {
        is_editing.set(field);
    };

    let open_role_menu = move |e: MouseEvent| {
        role_menu_open.set(!role_menu_open());
    };

    let edit_roles = move |id, remove| {
        spawn_local(async move {
            let assignment = if remove {
                unsign_role_from_user(id, user.id).await
            } else {
                assign_role_to_user(id, user.id).await
            };
            match assignment {
                Ok(_) => {
                    user_roles.refetch()
                },
                Err(msg) => message.set(msg.to_string())
            }
        })
    };

    let on_mouse_role = move |id, state| {
        let id_name = format!("role-{}", id);
        if id != 1 {
            if let Some(true) = {
                if let Some(perms) = user_page_perms.get() {
                    Some(perms["GerenciarCargos"])
                } else {
                    Some(false)
                }
            } {
                let minus = document().get_element_by_id(&id_name).unwrap();
                
                if state == "over" {
                    let _ = minus.class_list().remove_1("hidden");
                } else if state == "out" {
                    let _ = minus.class_list().add_1("hidden");

                }
            }
        }
    };

    let on_mouse_editable = move |identifier, state| {
        let editable_content = document().get_element_by_id(identifier).unwrap();
        if state == "over" {
            let _ = editable_content.class_list().remove_1("hidden");
        } else if state == "out" {
            let _ = editable_content.class_list().add_1("hidden");
        }
    };

    let save_fields = move |field| {
        match field {
            "field-1" => {
                spawn_local(async move {
                    if username() != username_backup() {
                        let user_updated = change_username(username()).await;
                        match user_updated {
                            Ok(user) => {
                                if user.username == username() {
                                    username_backup.set(user.username);
                                    message.set(String::new());
                                    is_editing.set("")
                                } else {
                                    username.set(username_backup());
                                    message.set("Algo deu errado, tente novamente.".to_string());
                                }
                            },

                            Err(msg) => {
                                username.set(username_backup());
                                message.set(msg.to_string());
                                is_editing.set("")
                            }
                        }
                    } else {
                        is_editing.set("")
                    }
                })
            },

            "field-2" => {
                log!("field-2 is saved");
                spawn_local(async move {
                    if email() != email_backup() {
                        let user_updated = change_email(email()).await;
                        match user_updated {
                            Ok(user) => {
                                if user.email == email() {
                                    email_backup.set(user.email);
                                    message.set(String::new());
                                    is_editing.set("")
                                } else {
                                    email.set(email_backup());
                                    message.set("Algo deu errado, tente novamente.".to_string());
                                }
                            },

                            Err(msg) => {
                                email.set(username_backup());
                                message.set(msg.to_string());
                                is_editing.set("")
                            }
                        }
                    } else {
                        message.set(String::new());
                        is_editing.set("")
                    }
                })
            },

            "field-3" => {
                if password() == password_confirm() {
                    spawn_local(async move {
                        let password_updated = edit_password(password()).await;
                        match password_updated {
                            Ok(_) => {
                                password.set(String::new());
                                password_confirm.set(String::new());
                                message.set(String::new());
                                is_editing.set("");
                            },

                            Err(msg) => {
                                password.set(String::new());
                                password_confirm.set(String::new());
                                message.set(msg.to_string());
                                is_editing.set("");
                            }
                        }
                    })
                } else {
                    message.set("As senhas não correspondem em ambos os campos.".to_owned())
                }
            },

            _ => {
                todo!()
            }
        }
    };

    view! {
        <main class="flex flex-col md:flex-row justify-center w-full bg-slate-200 min-h-screen">
            <Transition fallback=move || {
                view! {
                    <div class="md:mx-2 md:w-1/2 my-4 self-center md:max-w-full max-w-[90%] md:self-start bg-white shadow-md">
                        <h1 class="text-left my-2 py-4 m-2 px-8 animate-pulse bg-gray-500">""</h1>
                        <div class="text-left my-2">
                            <span class="animate-pulse rounded-full px-6 py-2 bg-gray-500"></span>
                            <span class="animate-pulse rounded-full px-6 py-2 bg-gray-500"></span>
                        </div>
                        <h2 class="py-3 px-12"></h2>
                        <hr class="mx-1"/>
                        <div class="m-4">
                            <p class="animate-pulse text-left py-2 px-6 mb-2 bg-gray-500"></p>
                            <p class="animate-pulse text-left py-4 px-8 bg-gray-500"></p>
                        </div>
                        <div class="m-4">
                            <p class="animate-pulse text-left py-2 px-6 mb-2 bg-gray-500"></p>
                            <p class="animate-pulse text-left py-4 px-8 bg-gray-500"></p>
                        </div>
                    </div>
                }
            }>
                <div class="md:mx-2 md:w-1/2 my-4 self-center md:max-w-full max-w-[90%] md:self-start bg-white shadow-md">
                    <Show
                        when=move || { is_editing() != "field-1" }
                        fallback=move || {
                            view! {
                                <p class="text-left ml-2 mx-4">
                                    <input
                                        type="text"
                                        on:input=move |ev| { username.set(event_target_value(&ev)) }
                                        prop:value=username
                                        class="border"
                                    />
                                    <button
                                        on:click=move |_| { save_fields("field-1") }
                                        class="rounded-md hover:shadow-[0_2px_8px_0_rgba(0,0,0,0.5)] transition-all bg-[#ef4444] hover:bg-[#d83c3c] text-white p-2 m-5"
                                    >
                                        "Salvar"
                                    </button>
                                </p>
                            }
                        }
                    >

                        <h1
                            id="field-1"
                            on:mouseover=move |_| on_mouse_editable("editable-1", "over")
                            on:mouseout=move |_| on_mouse_editable("editable-1", "out")
                            class="text-left font-bold text-4xl ml-4"
                        >
                            {username}
                            <button
                                on:click=move |_| edit_field("field-1")
                                id="editable-1"
                                class="hidden ml-2"
                            >
                                <Icon icon=i::BiPencilSolid/>
                            </button>
                        </h1>
                    </Show>
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

                    <p class="text-left px-2 py-3">
                        {move || {
                            user_roles
                                .get()
                                .map(|roles| {
                                    roles
                                        .into_iter()
                                        .map(|role| {
                                            view! {
                                                <span
                                                    on:mouseover=move |_| on_mouse_role(role.id, "over")
                                                    on:mouseout=move |_| on_mouse_role(role.id, "out")
                                                    class="bg-gray-300 py-1 mr-[6px] rounded-full"
                                                >
                                                    <span class="px-2">{&role.name}</span>
                                                    <button
                                                        on:click=move |_| edit_roles(role.id, true)
                                                        id=format!("role-{}", role.id)
                                                        class=" px-4 rounded-full transition-colors py-[1px] -ml-1 text-white bg-red-500 hover:bg-red-400 hidden"
                                                    >
                                                        "-"
                                                    </button>
                                                </span>
                                            }
                                        })
                                        .collect_view()
                                })
                        }}
                        <Show when=move || {
                            if let Some(perms) = user_page_perms.get() {
                                perms["GerenciarCargos"] && is_valid_account()
                            } else {
                                false
                            }
                        }>

                            <button
                                on:click=open_role_menu
                                class="bg-gray-300 py-[2px] px-2 mr-[6px] rounded-full"
                            >
                                "+"
                            </button>
                        </Show>
                    </p>
                    <Show when=move || {
                        if let Some(perms) = user_page_perms.get() {
                            perms["GerenciarCargos"] && is_valid_account()
                        } else {
                            false
                        }
                    }>

                        <AnimatedShow
                            when=role_menu_open
                            show_class="slideDown500"
                            hide_class="slideUp500"
                            hide_delay=Duration::from_millis(500)
                        >
                            <div class="bg-gray-100">
                                <div class="p-2 overflow-x-auto text-start whitespace-nowrap">
                                    {move || {
                                        let disabled = user_roles.get().unwrap();
                                        system_roles
                                            .get()
                                            .map(|roles| {
                                                view! {
                                                    {move || {
                                                        roles
                                                            .iter()
                                                            .map(|role| {
                                                                if !disabled.contains(&role) {
                                                                    let role_id = role.id;
                                                                    view! {
                                                                        <button
                                                                            on:click=move |_| edit_roles(role_id, false)
                                                                            class="bg-gray-300 py-1 px-2 mr-[6px] rounded-full"
                                                                        >
                                                                            {&role.name}
                                                                        </button>
                                                                    }
                                                                } else {
                                                                    view! {
                                                                        <button
                                                                            disabled=true
                                                                            class="bg-gray-200 text-gray-400 py-1 px-2 mr-[6px] rounded-full"
                                                                        >
                                                                            {&role.name}
                                                                        </button>
                                                                    }
                                                                }
                                                            })
                                                            .collect_view()
                                                    }}

                                                    <button class="bg-gray-300 py-1 px-2 mr-[6px] text-gray-500 rounded-full">
                                                        "Criar novo cargo..."
                                                    </button>
                                                }
                                            })
                                    }}

                                </div>
                            </div>
                        </AnimatedShow>
                    </Show>
                    <h2 class="font-bold text-2xl">"Informações de Cadastro"</h2>
                    <hr class="mx-2 mb-2"/>
                    <div class="p-2">
                        <Show
                            when=move || { is_editing() != "field-2" }
                            fallback=move || {
                                view! {
                                    <p class="text-left ml-2 mx-4">
                                        <p class="text-left text-gray-500 mb-2">"E-mail"</p>
                                        <input
                                            type="text"
                                            on:input=move |ev| { email.set(event_target_value(&ev)) }
                                            prop:value=email
                                            class="border"
                                        />
                                        <button
                                            on:click=move |_| { save_fields("field-2") }
                                            class="rounded-md hover:shadow-[0_2px_8px_0_rgba(0,0,0,0.5)] transition-all bg-[#ef4444] hover:bg-[#d83c3c] text-white p-2 m-5"
                                        >
                                            "Salvar"
                                        </button>
                                    </p>
                                }
                            }
                        >
                            <p class="text-left text-gray-500 mb-2">"E-mail"</p>
                            <p
                                on:mouseover=move |_| on_mouse_editable("editable-2", "over")
                                on:mouseout=move |_| on_mouse_editable("editable-2", "out")
                                class="text-left text-lg"
                            >
                                {email}
                                <button 
                                    on:click=move |_| edit_field("field-2")
                                    id="editable-2" 
                                    class="hidden">
                                    <Icon icon=i::BiPencilSolid/>
                                </button>
                            </p>
                        </Show>
                    </div>
                    <div class="p-2">
                        <Show
                            when=move || { is_editing() != "field-3" }
                            fallback=move || {
                                view! {
                                    <p class="text-left ml-2 mx-4">
                                        <p class="my-2">
                                            <p class="text-left text-gray-500 mb-2">"Nova Senha"</p>
                                            <input
                                                type="password"
                                                on:input=move |ev| { password.set(event_target_value(&ev)) }
                                                prop:value=password
                                                class="border"
                                            />
                                        </p>
                                        <p class="my-2">
                                            <p class="text-left text-gray-500 mb-2">"Repita a Senha"</p>
                                            <input
                                                type="password"
                                                on:input=move |ev| { password_confirm.set(event_target_value(&ev)) }
                                                prop:value=password_confirm
                                                class="border"
                                            />
                                        </p>
                                        <p class="my-5">
                                            <button
                                                on:click=move |_| { is_editing.set("") }
                                                class="rounded-md hover:shadow-[0_2px_8px_0_rgba(0,0,0,0.5)] transition-all bg-[#ef4444] hover:bg-[#d83c3c] text-white p-2 mx-2"
                                            >
                                                "Cancelar"
                                            </button>
                                            <button
                                                on:click=move |_| { save_fields("field-3") }
                                                class="rounded-md hover:shadow-[0_2px_8px_0_rgba(0,0,0,0.5)] transition-all bg-[#ef4444] hover:bg-[#d83c3c] text-white p-2"
                                            >
                                                "Salvar"
                                            </button>
                                        </p>
                                    </p>
                                }
                            }
                        >
                            <p class="text-left text-gray-500 mb-2">"Senha de Acesso"</p>
                            <p
                                on:mouseover=move |_| on_mouse_editable("editable-3", "over")
                                on:mouseout=move |_| on_mouse_editable("editable-3", "out")
                                class="text-left text-lg"
                            >
                                "*************"
                                <button 
                                    on:click=move |_| edit_field("field-3")
                                    id="editable-3" 
                                    class="hidden">
                                    <Icon icon=i::BiPencilSolid/>
                                </button>
                            </p>
                        </Show>
                    </div>
                    <div class="p-2">
                        <p class="text-left text-gray-500 mb-2">"Entrou em "</p>
                        <p class="text-left text-lg">
                            {move || format!("{}", user.created_at.format("%d/%m/%Y %H:%M"))}
                        </p>
                    </div>
                    <h3 class="font-bold text-xl">"Detalhes da Assinatura"</h3>
                    <div class="m-2 border">
                        {move || {
                            user_membership
                                .get()
                                .map(|membership| {
                                    match membership {
                                        Some(membership) => {
                                            if membership.is_lifetime {
                                                view! {
                                                    <>
                                                        <p class="py-16 text-2xl text-gray-500">
                                                            "Essa conta possui acesso vitalício!"
                                                        </p>
                                                    </>
                                                }
                                            } else {
                                                view! {
                                                    <>
                                                        <div class="p-2">
                                                            <p class="text-left text-gray-500 mb-2">
                                                                "Sua inscrição expira em "
                                                            </p>
                                                            <p class="text-left text-lg">
                                                                {move || {
                                                                    format!(
                                                                        "{}",
                                                                        membership.expires_at.format("%d/%m/%Y %H:%M"),
                                                                    )
                                                                }}
                                                            </p>
                                                        </div>
                                                    </>
                                                }
                                            }
                                        }
                                        None => {
                                            view! {
                                                <>
                                                    <Redirect path="login"/>
                                                </>
                                            }
                                        }
                                    }
                                })
                        }}

                    </div>
                    <Show when=move || {
                        if let Some(perms) = user_page_perms.get() {
                            for (_, perm) in perms {
                                if perm {
                                    return true && is_valid_account();
                                }
                            }
                            false
                        } else {
                            false
                        }
                    }>

                        <h2 class="font-bold text-2xl">"Ferramentas de Administração"</h2>
                        <Show when=move || {
                            if let Some(perms) = user_page_perms.get() {
                                ( perms["EditarUsuários"] || perms["ModerarUsuários"] ) && is_valid_account()
                            } else {
                                false
                            }
                        }>

                            <button class="rounded-md hover:shadow-[0_2px_8px_0_rgba(0,0,0,0.5)] transition-all bg-[#ef4444] hover:bg-[#d83c3c] text-white p-2 m-5">
                                "Visualizar Usuários"
                            </button>
                        </Show>
                    </Show>
                </div>
                <div class="md:mx-2 md:w-1/2 my-4 self-center md:max-w-full max-w-[90%] md:self-start bg-white shadow-md">
                    <h2 class="font-bold text-2xl">"Meus Artigos"</h2>
                    <hr class="mx-1"/>
                    <p class="py-16 text-2xl text-gray-500">"Recurso em Desenvolvimento!"</p>
                </div>
            </Transition>
        </main>
    }
}

/// Renders the home page of your application.
#[component]
pub fn Profile() -> impl IntoView {
    let user = create_blocking_resource(move || (), move |_| async move {
        let user = get_logged_user().await;
        user
    });
    view! {
        <Title text="Meu perfil"/>
        <Transition>
            {move || {
                user.get()
                    .map(|user| match user {
                        Ok(user) => {
                            view! {
                                <>
                                    <UserInfo user/>
                                </>
                            }
                        }
                        Err(msg) => {
                            view! {
                                <>
                                    <h1>{msg.to_string()}</h1>
                                    <Redirect path="/login"/>
                                </>
                            }
                        }
                    })
            }}

        </Transition>
    }
}