use std::{collections::HashMap, time::Duration};

use leptos::*;
use leptos_icons::*;
use icondata as i;
use leptos_meta::*;
use leptos_router::*;

use crate::functions::{get_jobs, get_logged_user, is_user_logged_in, user_have_permission, JobModel, UserModel};


/// Renders the home page of your application.
#[component]
pub fn JobsPage() -> impl IntoView {
    let jobs = create_resource(|| (), |_| async move {get_jobs().await});

    let search_input = create_rw_signal(String::new());

    let highlight_content = move |content: String| {
        if search_input().is_empty() {
            return view! { <span>{content}</span> }
        }

        let start_index = match content.to_lowercase().find(&search_input().to_lowercase()) {
            Some(index) => index,
            None => return view! { <span>{content}</span> },
        };
    
        // Calcula o índice final do trecho de texto a ser substituído
        let end_index = start_index + search_input().len();
    
        // Substitui a substring pela tag span
        let mut filtered_content = String::new();
        filtered_content.push_str(&content[..start_index]);
        filtered_content.push_str("<span class=\"bg-yellow-200\">");
        filtered_content.push_str(&content[start_index..end_index]);
        filtered_content.push_str("</span>");
        filtered_content.push_str(&content[end_index..]);

        view! { <span inner_html=filtered_content></span> }
    };

    let is_logged = create_local_resource(|| (), |_| async move { is_user_logged_in().await.unwrap() });

    let user_page_perms = create_local_resource(move || is_logged.get(), |logged| async move {
        let mut user_perms = HashMap::new();
        if let Some(true) = logged {
            let logged_user = get_logged_user().await;
            match logged_user {
                Ok(user) => user_perms.extend(
                    vec![
                            ("Confiavel".to_owned(), user.is_confirmed.clone()),
                            ("EditarVagas".to_owned(), user_have_permission(user.id, "EditarVagas".to_owned()).await.unwrap()),
                        ]
                    ),
                _ => unreachable!("Usuário não está conectado embora a diretiva tenha retornado verdadeiro.")
            }
        } else {
            user_perms.extend(
                vec![
                        ("Confiavel".to_owned(), false),
                        ("EditarVagas".to_owned(), false),
                    ]
                )
        }
        user_perms
    });

    let on_mouse_job = move |id: i32, state: &'static str| {
        let id_delete = format!("job-delete-{}", id);
        let id_edit = format!("job-edit-{}", id);
        
        if let Some(true) = {
            if let Some(perms) = user_page_perms.get() {
                Some(perms["Confiavel"] && perms["EditarVagas"])
            } else {
                Some(false)
            }
        } {
            let element_delete = document().get_element_by_id(&id_delete).unwrap();
            let element_edit = document().get_element_by_id(&id_edit).unwrap();


            if state == "over" {
                let _ = element_delete.class_list().remove_1("opacity-0");
                let _ = element_edit.class_list().remove_1("opacity-0");
            } else if state == "out" {
                let _ = element_delete.class_list().add_1("opacity-0");
                let _ = element_edit.class_list().add_1("opacity-0");
            }
        }
        
    };

    let should_delete = create_rw_signal(-1);

    let create_job = move |_| {
        let navigate = use_navigate();
        if search_input().is_empty() {
            navigate("/editor/vagas/criar", Default::default());
        } else {
            navigate(&format!("/editor/vagas/criar?use_name={}", search_input()), Default::default());
        }
    };

    view! {
        <Title text="Vagas"/>
        <main class="flex justify-center w-full bg-slate-200 min-h-screen">
            <div class="mx-2 md:w-1/2 my-4 w-full self-start bg-white shadow-md">
                <div class="border-2 rounded-md my-6 max-w-16 mx-3 shadow-md flex">
                    <input
                        on:input=move |ev| { search_input.set(event_target_value(&ev)) }
                        prop:value=search_input
                        class="flex-grow px-2 focus:outline-none"
                        type="search"
                    />
                    <button class="p-2 cursor-default border-l-2">
                        <Icon icon=i::HiMagnifyingGlassOutlineLg/>
                    </button>
                </div>
                <Transition>
                    <Show when=move || {
                        if let Some(perms) = user_page_perms.get() {
                            perms["EditarVagas"] && perms["Confiavel"]
                        } else {
                            false
                        }
                    }>
                        <button on:click=create_job class="rounded-md my-2 hover:shadow-[0_2px_8px_0_rgba(0,0,0,0.5)] transition-all bg-[#ef4444] hover:bg-[#d83c3c] text-white p-2 min-w-[90%]">
                            "Criar Nova Vaga"
                        </button>
                    </Show>
                </Transition>
                <div>
                    <Transition fallback=move || {
                        view! { <p>"Carregando..."</p> }
                    }>
                        {move || {
                            jobs.get()
                                .map(|data| match data {
                                    Ok(jobs) => {
                                        view! {
                                            <ul>
                                                {jobs
                                                    .iter()
                                                    .filter(|job| {
                                                        if search_input().len() >= 1 {
                                                            job.position
                                                                .to_lowercase()
                                                                .contains(&search_input().trim().to_ascii_lowercase())
                                                        } else {
                                                            true
                                                        }
                                                    })
                                                    .map(|job_model| {
                                                        let job_id = create_rw_signal(job_model.id.to_owned());
                                                        let job_name = create_rw_signal(job_model.position.clone());
                                                        view! {
                                                            <li
                                                                on:mouseover=move |_| on_mouse_job(job_id(), "over")
                                                                on:mouseout=move |_| on_mouse_job(job_id(), "out")
                                                                class="m-4"
                                                            >
                                                                <div class="rounded-md border-2">
                                                                    <div class="flex">
                                                                        <h2 class="flex-grow px-1 font-bold text-lg text-left">
                                                                            <A class="hover:underline" href=format!("/vagas/informacoes?id={}", job_id())>{highlight_content(job_name())}</A>
                                                                        </h2>
                                                                        <Show when=move || {
                                                                            if let Some(perms) = user_page_perms.get() {
                                                                                perms["EditarVagas"]
                                                                            } else {
                                                                                false
                                                                            }
                                                                        }>

                                                                            <div class="px-1">
                                                                                <button
                                                                                    on:click=move |_| {
                                                                                        let navigate = use_navigate();
                                                                                        navigate(&format!("/editor/vagas/{}", job_id()), Default::default());
                                                                                    }
                                                                                    class="transition opacity-0 p-1 m-1 rounded-md border-transparent border text-gray-500 hover:text-blue-600 hover:border-blue-600"
                                                                                    id=format!("job-edit-{}", job_id())
                                                                                >
                                                                                    <Icon height="24px" width="24px" icon=i::BiPencilSolid/>
                                                                                </button>
                                                                                <button
                                                                                    on:click=move |_| {
                                                                                        should_delete.set(job_id());
                                                                                    }
                                                                                    class="transition opacity-0 p-1 m-1 rounded-md text-gray-500 hover:bg-red-600 hover:text-white"
                                                                                    id=format!("job-delete-{}", job_id())
                                                                                >
                                                                                    <Icon height="24px" width="24px" icon=i::BiTrashRegular/>
                                                                                </button>
                                                                            </div>
                                                                        </Show>
                                                                    </div>
                                                                    <div class="flex">
                                                                        <p class="text-gray-500 flex-grow text-left pl-1">
                                                                            "Oferecido por "
                                                                            <span class="font-bold">{&job_model.company}</span>
                                                                        </p>
                                                                        <p class="text-gray-500 flex px-1 content-center">
                                                                            <span>
                                                                                {format!("{}", job_model.published_at.format("%d/%m/%Y"))}
                                                                            </span>
                                                                            <Icon
                                                                                height="1.5rem"
                                                                                width="1.5rem"
                                                                                class="my-auto mx-1 h-6 w-auto"
                                                                                icon=i::BiTimeRegular
                                                                            />
                                                                        </p>
                                                                    </div>
                                                                    <AnimatedShow
                                                                    when=(move || should_delete() == job_id()).into_signal()
                                                                    show_class="slideDown500"
                                                                    hide_class="slideUp500"
                                                                    hide_delay=Duration::from_millis(500)
                                                                    >
                                                                        <div class="bg-gray-100 py-2">
                                                                            <p>"Tem certeza que deseja apagar essa vaga? Essa ação é irreversível!"</p>
                                                                            <div class="flex mb-1">
                                                                                <button class="rounded-md hover:shadow-[0_2px_8px_0_rgba(0,0,0,0.5)] transition-all bg-[#ef4444] hover:bg-[#d83c3c] text-white p-2 w-full mx-1">
                                                                                    "Sim"
                                                                                </button>
                                                                                <button class="rounded-md hover:shadow-[0_2px_8px_0_rgba(239,68,68,0.5)] transition-all border-[#ef4444] hover:bg-white hover:border-[#d83c3c] hover:text-[#d83c3c] text-[#ef4444] border p-2 w-full mx-1">
                                                                                    "Não"
                                                                                </button>
                                                                            </div>
                                                                        </div>
                                                                    </AnimatedShow>
                                                                </div>
                                                            </li>
                                                        }
                                                    })
                                                    .collect_view()}

                                            </ul>
                                        }
                                            .into_view()
                                    }
                                    _ => view! { <pre>"Error"</pre> }.into_view(),
                                })
                        }}

                    </Transition>
                </div>
            </div>
        </main>
    }
}