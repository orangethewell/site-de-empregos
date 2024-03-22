use leptos::{ev::MouseEvent, html::ToHtmlElement, *};
use leptos_meta::*;
use leptos_router::*;
use leptos_icons::*;
use icondata as i;

use crate::functions::{add_job, edit_job, get_job, JobModel};

#[derive(Params, PartialEq)]
struct JobsEditorParams {
    id: i32
}

#[derive(Params, PartialEq)]
struct JobsEditorQuery {
    use_name: String
}

#[component]
pub fn JobsEditor() -> impl IntoView {
    let params = use_params::<JobsEditorParams>();
    let query = use_query::<JobsEditorQuery>();

    let tab_id = create_rw_signal(0);

    let id = move || {
        params.with(|params| {
            params.as_ref()
                .map(|params| params.id)
                .unwrap_or(-1)
        })
    };

    let use_name = move || {
        query.with(|query| {
            query.as_ref()
                .map(|query| query.use_name.clone())
                .unwrap_or(String::new())
        })
    };

    let is_create_mode = move || {
        id() == -1
    };

    let title_mode = move || {
        if is_create_mode() {
            "Criar uma Vaga"
        } else {
            "Editar uma Vaga"
        }
    };

    // Form values ------
    let position = create_rw_signal(use_name());
    let company = create_rw_signal(String::new());

    let requirements = create_rw_signal(Vec::<String>::new());
    let new_requirement = create_rw_signal(String::new());
    let current_editable_requirement = create_rw_signal(String::new());
    let requirement_id = create_rw_signal(-1);

    let description = create_rw_signal(String::new());
    // ------------------
    if !is_create_mode() {
        let job_details = create_local_resource(id, get_job);
        create_effect(move |_| {
            match job_details.get() {
                Some(job) => {
                    match job {
                        Ok(job_info) => {
                            position.set(job_info.position);
                            company.set(job_info.company);
                            requirements.set(job_info.requirements);
                            description.set(job_info.description.unwrap_or_default());
                        },
                        Err(_) => {}
                    }
                },
                None => {}
            }
        });
    }

    let show_pre = move || {
        !position().is_empty() || !company().is_empty() || !requirements().is_empty() || 
        !new_requirement().is_empty() || !description().is_empty()
    };

    let add_requirement = move |e: MouseEvent| {
        e.prevent_default();
        current_editable_requirement.set("".to_owned());
        requirement_id.set(-1);
        if !new_requirement().is_empty() {
            requirements.update(|reqs| reqs.push(new_requirement()));
            new_requirement.set("".to_owned())
        }
    };

    let on_mouse_req = move |id: i32, state: &'static str| {
        let id_delete = format!("req-delete-{}", id);
        let id_edit = format!("req-edit-{}", id);
        
        let element_delete = document().get_element_by_id(&id_delete).unwrap();
        let element_edit = document().get_element_by_id(&id_edit).unwrap();

        if state == "over" {
            let _ = element_delete.class_list().remove_1("opacity-0");
            let _ = element_edit.class_list().remove_1("opacity-0");
        } else if state == "out" {
            let _ = element_delete.class_list().add_1("opacity-0");
            let _ = element_edit.class_list().add_1("opacity-0");
        }
        
    };

    let edit_requirement = move |id| {
        current_editable_requirement.set(requirements()[id as usize].clone());
        requirement_id.set(id);
    };

    let delete_requirement = move |id| {
        current_editable_requirement.set("".to_owned());
        requirement_id.set(-1);
        requirements.update(|reqs| {reqs.remove(id as usize);});
    };

    let save_requirement_edits = move |id| {
        if !current_editable_requirement().is_empty() {
            requirements.update(|reqs| reqs[id as usize] = current_editable_requirement());
            current_editable_requirement.set("".to_owned());
            requirement_id.set(-1);
        } else {
            delete_requirement(id);
        }
    };

    let save_job = move |ev: MouseEvent| {
        ev.prevent_default();
        let mut model = JobModel::new(position(), company(), Some(description()), requirements());
        if is_create_mode() {
            spawn_local(async move {
                match add_job(model).await {
                    Ok(job) => {window().location().set_href("/vagas");},
                    Err(_) => {}

                }
            });
        } else {
            model.id = id();
            spawn_local(async move {
                match edit_job(model).await {
                    Ok(job) => {window().location().set_href("/vagas");},
                    Err(_) => {}
                }
            });
        }
    };

    view! {
        <Title text="Editor de Vagas"/>
        <main class="flex justify-center w-full bg-slate-200 min-h-screen">
            <div class="mx-2 md:w-1/2 my-4 w-full self-start">
            <div class="text-left">
                <button on:click=move |_| tab_id.set(0) class=("bg-white", move || tab_id() == 0) class=("bg-gray-300", move || tab_id() != 0) class="transition rounded-t-lg px-2 ml-2 mr-1">"Editor"</button>
                <button on:click=move |_| tab_id.set(1) class=("bg-white", move || tab_id() == 1) class=("bg-gray-300", move || tab_id() != 1) class="transition rounded-t-lg px-2 mr-1">"Pré-visualização"</button>
            </div>
            <div class="bg-white shadow-md">
                <Show 
                    when=move || tab_id() == 0
                >
                    <form class="p-4">
                        <h1 class="text-3xl font-bold mb-2">{title_mode()}</h1>
                        <hr class="mx-4 mb-6 h-[2px] bg-gray-300"/>
                            <Show
                            when=show_pre
                            >
                                <div class="rounded-md border-2 m-4">
                                    <div class="flex">
                                        <h2 class="flex-grow px-1 font-bold text-lg text-left">
                                            {move || if position().is_empty() {"Sem nome".to_owned()} else { position()}}
                                        </h2>
                                    </div>
                                    <div class="flex">
                                        <p class="text-gray-500 flex-grow text-left pl-1">
                                            "Oferecido por "
                                            <span class="font-bold">{move || if company().is_empty() {"...".to_owned()} else { company()}}</span>
                                        </p>
                                    </div>
                                </div>
                        </Show>
                        <div class="mb-4">
                            <p class="text-left font-bold">"Cargo"</p>
                            <input
                                on:input=move |ev| {
                                    position.set(event_target_value(&ev))
                                }
                                prop:value=position
                                id="position"
                                class="rounded-md border-2 w-full border-gray-400 flex-grow p-2 focus:outline-none focus:border-gray-600"
                                type="text"
                            />
                        </div>
                        <div class="mb-4">
                            <p class="text-left font-bold">"Empresa Solicitante"</p>
                            <input
                                on:input=move |ev| {
                                    company.set(event_target_value(&ev))
                                }
                                prop:value=company
                                id="company"
                                class="rounded-md border-2 w-full border-gray-400 flex-grow p-2 focus:outline-none focus:border-gray-600"
                                type="text"
                            />
                        </div>
                        <div class="mb-4">
                            <p class="text-left font-bold">"Requisitos"</p>
                            <Show
                                when=move || {!requirements().is_empty()}
                            >
                                <ul class="my-2 mx-4 text-left list-disc">
                                    {move || {
                                        requirements()
                                            .iter()
                                            .enumerate()
                                            .map(|(index, value)| {
                                                let tmp_index = create_rw_signal(index.clone() as i32);
                                                let tmp_value = create_rw_signal(value.clone());
                                                view! {
                                                    <li 
                                                    on:mouseover=move |_| on_mouse_req(tmp_index(), "over") 
                                                    on:mouseout=move |_| on_mouse_req(tmp_index(), "out")
                                                    class=("border-y-2", move || tmp_index() == 0) 
                                                    class=("border-b-2", move || tmp_index() > 0)
                                                    class="p-2 items-center flex">
                                                        <Show fallback=move || {
                                                            view! {
                                                                <span class="flex-grow">{tmp_value()}</span>
                                                                <div class="px-1">
                                                                    <button
                                                                        on:click=move |e| {e.prevent_default(); edit_requirement(tmp_index())}
                                                                        class="transition opacity-0 p-1 m-1 rounded-md border-transparent border text-gray-500 hover:text-blue-600 hover:border-blue-600"
                                                                        id=format!("req-edit-{}", tmp_index())
                                                                    >
                                                                        <Icon height="24px" width="24px" icon=i::BiPencilSolid/>
                                                                    </button>
                                                                    <button
                                                                        on:click=move |e| {e.prevent_default(); delete_requirement(tmp_index())}
                                                                        class="transition opacity-0 p-1 m-1 rounded-md text-gray-500 hover:bg-red-600 hover:text-white"
                                                                        id=format!("req-delete-{}", tmp_index())
                                                                    >
                                                                        <Icon height="24px" width="24px" icon=i::BiTrashRegular/>
                                                                    </button>
                                                                </div>
                                                            }
                                                        } when=move || requirement_id() == tmp_index()>
                                                            <input
                                                            on:input=move |ev| {
                                                                current_editable_requirement.set(event_target_value(&ev))
                                                            }
                                                            on:keypress=move |ev| {
                                                                if ev.key() == "Enter" {
                                                                    ev.prevent_default();
                                                                    let el = document().get_element_by_id("save-requirement").unwrap().to_leptos_element();
                                                                    el.click();
                                                                }
                                                            }
                                                            prop:value=current_editable_requirement
                                                            id="editable_requirement"
                                                            class="rounded-md border-2 w-full border-gray-400 flex-grow p-2 focus:outline-none focus:border-gray-600"
                                                            type="text"
                                                            />
                                                            <button
                                                            id="save-requirement"
                                                            on:click=move |e| {e.prevent_default(); save_requirement_edits(tmp_index())}
                                                            class="rounded-md hover:shadow-[0_2px_8px_0_rgba(0,0,0,0.5)] transition-all bg-[#ef4444] hover:bg-[#d83c3c] ml-2 text-white p-2">
                                                                "Salvar"
                                                            </button>
                                                        </Show>
                                                    </li>
                                                }
                                            })
                                            .collect_view()
                                    }}
                                </ul>
                            </Show>
                            <div class="flex">
                                <input
                                    on:keypress=move |ev| {
                                        if ev.key() == "Enter" {
                                            ev.prevent_default();
                                            let el = document().get_element_by_id("add-requirement").unwrap().to_leptos_element();
                                            el.click();
                                        }
                                    }
                                    on:input=move |ev| {
                                        new_requirement.set(event_target_value(&ev))
                                    }
                                    prop:value=new_requirement
                                    id="requirements"
                                    class="rounded-md border-2 w-full border-gray-400 flex-grow p-2 focus:outline-none focus:border-gray-600"
                                    type="text"
                                />
                                <button
                                on:click=add_requirement 
                                id="add-requirement"
                                class="rounded-md hover:shadow-[0_2px_8px_0_rgba(0,0,0,0.5)] transition-all bg-[#ef4444] hover:bg-[#d83c3c] ml-2 text-white p-2">
                                    "Adicionar"
                                </button>
                            </div>
                        </div>
                        <div class="mb-4">
                            <p class="text-left font-bold">"Descrição"</p>
                            <textarea 
                            prop:value=description 
                            on:input=move |ev| {
                                description.set(event_target_value(&ev))
                            }
                            class="rounded-md border-2 w-full border-gray-400 flex-grow p-2 focus:outline-none focus:border-gray-600"></textarea>
                        </div>
                        <button on:click=save_job class="rounded-md hover:shadow-[0_2px_8px_0_rgba(0,0,0,0.5)] transition-all bg-[#ef4444] hover:bg-[#d83c3c] text-white p-2 w-full mb-5">
                            "Salvar"
                        </button>
                    </form>
                </Show>
                <Show when=move || tab_id() == 1>
                    <div class="text-left">
                        <h1 class="mx-2 text-2xl font-bold my-1">{position}</h1>
                        <hr class="mx-3"/>
                        <div class="mx-3">
                            <h2 class="text-gray-500 font-bold my-2 text-lg">"Requisitos"</h2>
                            <div class="mx-8">
                                <ul class="list-disc">
                                    <For
                                        each=requirements
                                        key=move |requirement| {requirements().iter().position(|el| el == requirement).unwrap()}
                                        let:requirement
                                    >
                                        <li>{requirement}</li>
                                    </For>
                                </ul>
                            </div>
                            <h2 class="text-gray-500 font-bold my-2 text-lg">"Descrição"</h2>
                            {match description().is_empty() {
                                false => view! {
                                    <p class="mx-8 whitespace-pre-wrap">
                                        {description}
                                    </p>
                                }.into_view(),
                                true => view! {
                                    <div class="flex justify-center content-center py-20">
                                        <p class="text-gray-500 font-bold text-lg">"Nenhuma informação adicional."</p>
                                    </div>
                                }.into_view()
                            }}
                            <p class="text-right text-gray-500 my-2">"Vaga publicada por "<b class="font-bold">{company}</b></p>
                            <button
                                class="rounded-md hover:shadow-[0_2px_8px_0_rgba(0,0,0,0.5)] transition-all bg-[#ef4444] hover:bg-[#d83c3c] text-white p-2 w-full mb-5"
                            >
                                "Tenho interesse"
                            </button>
                        </div>
                    </div>
                </Show>
            </div>
            </div>
        </main>
    }
}