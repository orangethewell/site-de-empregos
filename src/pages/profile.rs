use std::time::Duration;

use leptos::{ev::MouseEvent, *};
use leptos_meta::*;
use leptos_router::*;
use leptos_icons::*;
use icondata as i;

use crate::functions::{assign_role_to_user, get_logged_user, get_roles, get_user_roles, user_have_permission, unsign_role_from_user, UserModel};


#[component]
pub fn UserInfo(user: UserModel) -> impl IntoView {
    let system_roles = create_resource(|| (), |_| async move {get_roles().await.unwrap()});
    let user_roles = create_resource(move || user.id, |id| async move {get_user_roles(id).await.unwrap()});
    
    let have_role_management_perm = create_resource(move || user.id, |id| async move {user_have_permission(id, "GerenciarCargos".to_owned()).await.unwrap()});

    let message = create_rw_signal(String::new());

    let role_menu_open = create_rw_signal(false);
    let open_role_menu = move |e: MouseEvent| {
        role_menu_open.set(!role_menu_open());
    };

    let on_add_role = move |id| {
        spawn_local(async move {
            let assignment = assign_role_to_user(id, user.id).await.unwrap();
            match assignment {
                Ok(_) => {user_roles.refetch(); system_roles.refetch()},
                Err(msg) => message.set(msg)
            }
        })
    };

    let on_remove_role = move |id| {
        spawn_local(async move {
            let assignment = unsign_role_from_user(id, user.id).await.unwrap();
            match assignment {
                Ok(_) => {user_roles.refetch(); system_roles.refetch()},
                Err(msg) => message.set(msg)
            }
        })
    };

    let on_mouseover_role = move |id| {
        let id_name = format!("role-{}", id);
        if id != 1 {
            if let Some(true) = have_role_management_perm.get() {
                let minus = document().get_element_by_id(&id_name).unwrap();
                
                let _ = minus.class_list().remove_1("hidden");
            }
        }
    };

    let on_mouseout_role = move |id| {
        let id_name = format!("role-{}", id);
        if id != 1 {
            if let Some(true) = have_role_management_perm.get() {
                let minus = document().get_element_by_id(&id_name).unwrap();
                
                let _ = minus.class_list().add_1("hidden");
            }
        }
    };

    let on_mouseout_editable = move |identifier| {
        let editable_content = document().get_element_by_id(identifier).unwrap();
        
        let _ = editable_content.class_list().add_1("hidden");

    };

    let on_mouseover_editable = move |identifier| {
        let editable_content = document().get_element_by_id(identifier).unwrap();
        
        let _ = editable_content.class_list().remove_1("hidden");
    };

    view! {
        <Transition fallback=move || view! {<h1>"Carregando..."</h1>}>
        <h1 on:mouseover=move |_| on_mouseover_editable("editable-1") on:mouseout=move |_| on_mouseout_editable("editable-1") class="text-left align-middle font-bold text-4xl ml-4">{&user.username}<button id="editable-1" class="hidden ml-2"><Icon icon=i::BiPencilSolid/></button></h1>
        {move || if !message().is_empty() {
                view! {
                    <><p class="border m-4 border-red-500 bg-red-200 text-red-500">{message()}</p></>
                }
            } else {
                view! {
                    <>{view!{}}</>
                }
            }
    
        }
        <p class="text-left px-2 py-3">
        {move || {
            user_roles.get()
                .map(|roles| { 
                    roles.iter().map(|role| {
                        let role_id = role.id;
                        view! {
                            <span on:mouseover=move |_| on_mouseover_role(role_id) on:mouseout=move |_| on_mouseout_role(role_id) class="bg-gray-300 py-1 mr-[6px] rounded-full"><span class="px-2">{&role.name}</span><button on:click=move |_| on_remove_role(role_id) id=format!("role-{}", role_id) class=" px-4 rounded-full transition-colors py-[1px] -ml-1 text-white bg-red-500 hover:bg-red-400 hidden">"-"</button></span>
                        }
                    }).collect_view()
                })
            }
        }
        {move || {    
            have_role_management_perm.get()
                .map(|permission_granted| {
                    if permission_granted {
                        view! {
                            <>
                            <button on:click=open_role_menu class="bg-gray-300 py-[2px] px-2 mr-[6px] rounded-full">"+"</button>
                            </>
                        }
                    } else {
                        view! {
                            <>
                            {view! {}}
                            </>
                        }
                    }
                })
        }}
        </p>
        {move || {    
            have_role_management_perm.get()
                .map(|permission_granted| {
                    if permission_granted {
                        view! {
                            <>
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
                                    system_roles.get()
                                        .map(|roles| {
                                            view! {
                                                { move || 
                                                roles.iter().map(|role| {
                                                    if !disabled.contains(&role) {
                                                        let role_id = role.id;
                                                        view! {
                                                            <button on:click=move |_| on_add_role(role_id) class="bg-gray-300 py-1 px-2 mr-[6px] rounded-full">{&role.name}</button>
                                                        }
                                                    } else {
                                                        view! {
                                                            <button disabled=true class="bg-gray-200 text-gray-400 py-1 px-2 mr-[6px] rounded-full">{&role.name}</button>
                                                        }
                                                    }
                                                }).collect_view()
                                                }
                                                <button class="bg-gray-300 py-1 px-2 mr-[6px] text-gray-500 rounded-full">"Criar novo cargo..."</button>
                                            }
                                        })
                                    }
                                }
                                </div>
                            </div>
                            </AnimatedShow>
                            </>
                        }
                    } else {
                        view! {
                            <>
                            {view! {}}
                            </>
                        }
                    }
                })
        }}
        <h2 class="font-bold text-2xl">"Informações de Cadastro"</h2>
        <hr class="mx-2 mb-2"/>
        <div class="p-2">
            <p class="text-left text-gray-500 mb-2">"E-mail"</p>
            <p on:mouseover=move |_| on_mouseover_editable("editable-2") on:mouseout=move |_| on_mouseout_editable("editable-2") class="text-left text-lg">{&user.email}<button id="editable-2" class="hidden"><Icon icon=i::BiPencilSolid/></button></p>
        </div>
        <div class="p-2">
            <p class="text-left text-gray-500 mb-2">"Entrou em "</p>
            <p class="text-left text-lg">{move || format!("{}", user.created_at.format("%d/%m/%Y %H:%M"))}</p>
        </div>
        <h3 class="font-bold text-xl">"Detalhes de Inscrição"</h3>
        <div class="m-2 border">
            
        </div>
        </Transition>
    }
}

/// Renders the home page of your application.
#[component]
pub fn Profile() -> impl IntoView {
    let user = create_blocking_resource(move || (), move |_| async move {
        let user = get_logged_user().await.unwrap();
        user
    });
    view! {
        <Title text="Meu perfil"/>
        <Transition fallback=move || view! {<h1>"Carregando..."</h1>}>
        <main class="flex justify-center w-full bg-slate-200 min-h-screen">
            <div class="mx-2 md:w-1/2 my-4 w-full self-start bg-white shadow-md">
                {move || {
                    user.get()
                        .map(|user| match user { 
                            Ok(user) => view! {
                                <>
                                <UserInfo user/>
                                </>
                            },
                            Err(msg) => view! {
                                <>
                                <h1>{msg}</h1>
                                <Redirect path="/login"/>
                                </>
                            }
                        })
                }}
            </div>
        </main>
        </Transition>
    }
}