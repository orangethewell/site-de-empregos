use bounce::use_atom;
use web_sys::HtmlInputElement;
use yew::{prelude::*, platform::spawn_local};
use yew_icons::{Icon, IconId};
use yew_notifications::use_notification;
use common::{Job, get_jobs, add_job, update_job, delete_job};
use crate::{components::{FloatWindow, TextInput, form_input::InputStatus, notifications::CustomNotification, menu::ShowMenu, AuthGuard, AuthFallback, AuthProtected}, app::Route};

#[derive(Properties, Clone, PartialEq)]
pub struct JobTableProps {
    pub jobs: Vec<Job>,
    pub on_add_job: Callback<MouseEvent>,
    pub on_edit_job: Callback<usize>,
    pub on_delete_job: Callback<usize>,
}

#[function_component(JobTable)]
pub fn job_table(props: &JobTableProps) -> Html {
    let JobTableProps {
        jobs,
        on_add_job,
        on_edit_job,
        on_delete_job,
    } = props;

    let loading_unit = html! {
        <div class="flex h-full items-center justify-center">
            <div
            class="inline-block h-8 w-8 animate-spin rounded-full border-4 border-solid border-current border-r-transparent align-[-0.125em] motion-reduce:animate-[spin_1.5s_linear_infinite]"
            role="status">
            <span
                class="!absolute !-m-px !h-px !w-px !overflow-hidden !whitespace-nowrap !border-0 !p-0 ![clip:rect(0,0,0,0)]"
                >{"Loading..."}
            </span>
            </div> 
        </div>
    };

    html! {
        <>
            <div class="overflow-auto p-5 max-h-[75vh] masked-overflow">
            {for jobs.iter().enumerate().map(|(index, job)| {
                let on_edit = on_edit_job.reform(move |_| index);
                let on_delete = on_delete_job.reform(move |_| index);

                html! {
                    <div class="bg-white relative rounded-lg shadow-xl p-4 flex mb-4">
                        <div class="flex-shrink-0">
                            <svg class="h-8 w-8 fill-current text-blue-500" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20">
                                <path d="M10 0a10 10 0 1 0 10 10A10 10 0 0 0 10 0zm0 18a8 8 0 1 1 8-8 8 8 0 0 1-8 8z"/>
                                <path d="M11 6a1 1 0 0 0-2 0v4a1 1 0 0 0 2 0V6z"/>
                            </svg>
                        </div>
                        <div class="ml-4 flex-grow">
                            <h2 class="text-lg font-semibold text-gray-800">{job.title.clone()}</h2>
                            <p class="text-gray-600">{job.branch.clone()}</p>
                        </div>
                        <AuthGuard loading_unit={loading_unit.clone()} permission="EditJobs">
                            <AuthProtected>
                                <button onclick={on_edit} class="flex-shrink mx-2"><Icon icon_id={IconId::BootstrapPencil} class="text-blue-600"/></button>
                                <button onclick={on_delete} class="flex-shrink mx-2"><Icon icon_id={IconId::BootstrapTrash} class="text-red-600"/></button>
                            </AuthProtected>
                        </AuthGuard>
                    </div>
                }
            })}
            </div>
            <AuthGuard loading_unit={loading_unit.clone()} permission="EditJobs">
                <AuthProtected>
                    <button onclick={on_add_job.clone()} class="flex p-4 bg-white border-4 border-gray-200 border-dashed rounded-lg justify-center w-full">{"Adicionar Vaga"}</button>
                </AuthProtected>
            </AuthGuard>
        </>
    }
}



#[derive(Properties, PartialEq)]
pub struct VectorViewerProps<T> 
where T: PartialEq + Clone {
    title: AttrValue,
    vector: Vec<T>,
    on_remove_value: Callback<usize>,
}

#[function_component(VectorViewer)]
pub fn vector_viewer<T: PartialEq + Clone + ToHtml + std::fmt::Display>(props: &VectorViewerProps<T>) -> Html {
    let items = props.vector.clone();
    let onremove = props.on_remove_value.clone();

    html!{
        <>
        if items.len() >= 1 {
            <div class="bg-gray-50 p-1 rounded-md">
                <h3 class="text-center mb-2 font-bold">{props.title.clone()}</h3>
                {items.into_iter().enumerate().map(|(index, value)| {
                    html! {
                        <div key={index} class="shadow-xl rounded-lg p-2 mb-2 bg-white flex">
                            <p class="flex-grow">{format!("-> {}", value)}</p>
                            <div class="flex-none"><button onclick={onremove.reform(move |e: MouseEvent| {e.prevent_default(); index})}><Icon icon_id={IconId::BootstrapTrash} class="flex-shrink-0 text-red-600"/></button></div>
                        </div>
                    }
                }).collect::<Html>()}
            </div>
        }
        </>
    }
}

#[derive(Clone)]
struct InputChecker {
    status: InputStatus,
    reference: NodeRef,
    data: String,
}

#[derive(Properties, PartialEq)]
pub struct JobEditorProps {
    #[prop_or(None)]
    job: Option<Job>,
    #[prop_or_default]
    oncomplete: Callback<MouseEvent>
}

fn save_checker_data(checkers: &mut Vec<InputChecker>) {
    for checker in checkers.iter_mut() {
        let data = checker.reference.cast::<HtmlInputElement>().unwrap().value();
        checker.data = data;
    }
}

#[function_component(JobEditor)]
pub fn job_editor(props: &JobEditorProps) -> Html {
    let job = use_state(|| props.job.clone().unwrap_or_default());
    let title = if props.job.is_some() {"Editar uma Vaga"} else {"Adicionar uma Vaga"};
    let requirements = use_state(|| job.requirements.clone());
    let activities = use_state(|| job.activities.clone());
    
    let notifier = use_notification::<CustomNotification>();
    
    let oncomplete = props.oncomplete.clone();
    let show_advice = use_state(|| false);

    let checkers = use_state(|| vec![
        InputChecker {status: InputStatus::Neutral, reference: NodeRef::default(), data: String::new()},
        InputChecker {status: InputStatus::Neutral, reference: NodeRef::default(), data: String::new()},
        InputChecker {status: InputStatus::Neutral, reference: NodeRef::default(), data: String::new()},
        InputChecker {status: InputStatus::Neutral, reference: NodeRef::default(), data: String::new()},
        InputChecker {status: InputStatus::Neutral, reference: NodeRef::default(), data: String::new()},
    ]);

    {
        let job = job.clone();
        let checkers = checkers.clone();
        use_effect_with((), move |_| {
            let mut updated_checkers = (*checkers).clone();
            updated_checkers[0].data = job.title.clone();
            updated_checkers[1].data = job.company.clone();
            updated_checkers[2].data = job.branch.clone();

            checkers.set(updated_checkers);
        })
    }

    let on_new_requirement = {
        let requirements = requirements.clone();
        let checkers = checkers.clone();
        Callback::from(move |event: KeyboardEvent| {
            if event.key() == "Enter" {
                event.prevent_default();
                let mut unstated_checkers = (*checkers).clone();
                save_checker_data(&mut unstated_checkers);
                unstated_checkers[3].data = "".to_owned(); 
                checkers.set(unstated_checkers);
                if let Some(input) = event.target_dyn_into::<HtmlInputElement>(){
                    let mut extended_require = (*requirements).clone();
                    extended_require.push(input.value().clone());
                    requirements.set(extended_require);
                    // input.set_value("");
                }
                
            }
        })
    };

    let on_remove_requirement = {
        let requirements = requirements.clone();
        let checkers = checkers.clone();
        Callback::from(move |index: usize| {
            let mut unstated_checkers = (*checkers).clone();
            save_checker_data(&mut unstated_checkers); 
            checkers.set(unstated_checkers);
            let mut updated_requirement = (*requirements).clone();
            updated_requirement.remove(index);
            requirements.set(updated_requirement)
        })
    };

    let on_remove_activity = {
        let activities = activities.clone();
        let checkers = checkers.clone();
        Callback::from(move |index: usize| {
            let mut unstated_checkers = (*checkers).clone();
            save_checker_data(&mut unstated_checkers); 
            checkers.set(unstated_checkers);
            let mut updated_activity = (*activities).clone();
            updated_activity.remove(index);
            activities.set(updated_activity)
        })
    };

    let on_new_activity = {
        let activities = activities.clone();
        let checkers = checkers.clone();
        Callback::from(move |event: KeyboardEvent| {
            if event.key() == "Enter" {
                event.prevent_default();
                let mut unstated_checkers = (*checkers).clone();
                save_checker_data(&mut unstated_checkers); 
                unstated_checkers[4].data = "".to_owned(); 
                checkers.set(unstated_checkers);
                if let Some(input) = event.target_dyn_into::<HtmlInputElement>(){
                    let mut extended_activity = (*activities).clone();
                    extended_activity.push(input.value().clone());
                    activities.set(extended_activity);
                }
            }
        })
    };

    let num_of_opportunities = use_state(|| job.opportunities);
    let on_add_opportunity = {
        let num_of_opportunities = num_of_opportunities.clone();
        let checkers = checkers.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let mut unstated_checkers = (*checkers).clone();
            save_checker_data(&mut unstated_checkers); 
            checkers.set(unstated_checkers);
            num_of_opportunities.set((*num_of_opportunities).clone() + 1);
        })
    };
    let on_sub_opportunity = {
        let num_of_opportunities = num_of_opportunities.clone();
        let checkers = checkers.clone();
        let show_advice = show_advice.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let mut unstated_checkers = (*checkers).clone();
            save_checker_data(&mut unstated_checkers); 
            checkers.set(unstated_checkers);
            if *num_of_opportunities > 0 {
                num_of_opportunities.set((*num_of_opportunities).clone() - 1);
            } 
            
            if *num_of_opportunities == 0 && title == "Editar uma Vaga" {
                show_advice.set(true)
            } else {
                show_advice.set(false)
            }
        })
    };

    let on_save_job = {
        let num_of_opportunities = num_of_opportunities.clone();
        let activities = activities.clone();
        let requirements = requirements.clone();
        let checkers = checkers.clone();

        let notifier = notifier.clone();
        let oncomplete = props.oncomplete.clone();
        let pass_job = job.clone();
        let show_advice = show_advice.clone();

        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let oncomplete_callback = oncomplete.clone();
            let notifier = notifier.clone();
            let mut should_save = true;
            let mut unstated_checkers = (*checkers).clone();
            let pass_job = pass_job.clone();
            
            if *num_of_opportunities == 0 && !*show_advice {
                notifier.spawn(CustomNotification::new("A vaga deve ter pelo menos uma oportunidade!".to_owned()));
                should_save = false;
            } else if *num_of_opportunities == 0 && *show_advice {
                should_save = false;
                let deleted_job = pass_job.clone();
                let remover_notifier = notifier.clone();
                let ondelete = oncomplete.clone();
                spawn_local(async move {
                    match delete_job(deleted_job.id).await {
                        Ok(()) => {
                            ondelete.emit(MouseEvent::new("click").unwrap());
                        },
                        Err(message) => remover_notifier.spawn(CustomNotification::new(&format!("Erro ao apagar vaga: {}", message)))
                    }
                })    
            }

            let mut values = vec![];

            for (i, checker) in unstated_checkers.iter_mut().enumerate() {
                if let Some(input) = checker.reference.cast::<HtmlInputElement>() {
                    match i {
                        0 | 1 | 2 => {
                            if input.value() != "" {
                                checker.status = InputStatus::Success;
                                values.push(input.value())
                            } else {
                                checker.status = InputStatus::Error;
                                should_save = false;
                                let identifier = match i {
                                    0 => "Título da vaga",
                                    1 => "Empresa",
                                    2 => "Ramo",
                                    _ => "Desconhecido"
                                };
                                notifier.spawn(CustomNotification::new(format!("O campo {} não foi preenchido corretamente.", identifier)))
                            }
                        },

                        3 => {
                            if requirements.len() == 0 {
                                checker.status = InputStatus::Error;
                                should_save = false;
                                notifier.spawn(CustomNotification::new("Você precisa adicionar pelo menos um requisito.".to_owned()));
                            } else {
                                checker.status = InputStatus::Success;
                            }
                        },

                        4 => {
                            if activities.len() == 0 {
                                checker.status = InputStatus::Error;
                                should_save = false;
                                notifier.spawn(CustomNotification::new("Você precisa adicionar pelo menos uma atividade.".to_owned()));
                            } else {
                                checker.status = InputStatus::Success;
                            }
                        },

                        _ => {
                            panic!("Unreachable pattern.")
                        }
                    }
                }
            }

            checkers.set(unstated_checkers);
            if should_save {
                let new_job = Job { 
                    activities: (*activities).clone(), 
                    title: values[0].clone(),
                    company: values[1].clone(),
                    branch: values[2].clone(),
                    id: -1, 
                    opportunities: *num_of_opportunities, 
                    requirements: (*requirements).clone(), 
                };
                spawn_local(async move {
                    let result = if pass_job.id != -1 && pass_job.title != "" {
                        update_job(pass_job.id, &new_job).await
                    } else {
                        add_job(&new_job).await
                    };
                    match result {
                        Ok(()) => {
                            oncomplete_callback.emit(MouseEvent::new("click").unwrap());
                        }
                        Err(err) => {notifier.spawn(CustomNotification::new(format!("Erro ao tentar adicionar/editar vaga: {}", err)))}
                    }
                });
                
            } else {
                ()
            }
        })
    };

    let prevent_exclusion = {
        Callback::from(|e: KeyboardEvent| {
            if e.key() == "Enter" {
                e.prevent_default();
            }
        })
    };

    html! {
        <>
            <h1 class="text-center text-2xl font-bold">{title.clone()}</h1>
            <form onsubmit={Callback::from(|e: SubmitEvent| e.prevent_default())}>
                <div class="flex p-3 rounded-lg">
                    <p class="flex-grow"><b>{"Oportunidades: "}</b>{*num_of_opportunities}</p>
                    <button class="px-2 mx-1 shadow-xl rounded-lg bg-red-400 text-white" onclick={on_add_opportunity}>{"+"}</button>
                    <button class="px-2 mx-1 shadow-xl rounded-lg bg-red-400 text-white" onclick={on_sub_opportunity}>{"-"}</button>
                </div>
                if *show_advice {
                    <p class="text-red-400 text-sm text-center">{"Essa vaga será apagada pois não tem nenhuma oportunidade!"}</p>
                }
                <TextInput maxlength="150" onkeypress={prevent_exclusion.clone()} input_ref={&checkers[0].reference} value={checkers[0].data.clone()} id="title" type_handler="text" status={checkers[0].status}>{"Título da Vaga"}</TextInput>
                <TextInput maxlength="150" onkeypress={prevent_exclusion.clone()} input_ref={&checkers[1].reference} value={checkers[1].data.clone()} id="company" type_handler="text" status={checkers[1].status}>{"Empresa"}</TextInput>
                <TextInput maxlength="150" onkeypress={prevent_exclusion.clone()} input_ref={&checkers[2].reference} value={checkers[2].data.clone()} id="branch" type_handler="text" status={checkers[2].status}>{"Ramo"}</TextInput>
                <VectorViewer<String> title="Requisitos" on_remove_value={on_remove_requirement} vector={(*requirements).clone()}/>
                <TextInput input_ref={&checkers[3].reference} id="requirement" type_handler="text" onkeypress={on_new_requirement} value={checkers[3].data.clone()} status={checkers[3].status}>{"Novo Requisito"}</TextInput>
                <VectorViewer<String> title="Atividades" on_remove_value={on_remove_activity} vector={(*activities).clone()}/>
                <TextInput input_ref={&checkers[4].reference} id="activity" type_handler="text" onkeypress={on_new_activity} value={checkers[4].data.clone()} status={checkers[4].status}>{"Nova Atividade"}</TextInput>
                <div class="flex sticky -bottom-4 bg-white py-2">
                    <button onclick={move |_| oncomplete.emit(MouseEvent::new("click").unwrap())} class="flex-grow border border-amber-700 rounded-xl text-amber-700 py-2">{"Cancelar"}</button>
                    <button onclick={on_save_job} class="flex-grow p-2 text-white rounded-xl mx-1 bg-red-500">{"Salvar"}</button>
                </div>
            </form>
        </>
    }
}

#[function_component(Jobs)]
pub fn jobs() -> Html {
    let show_role_editor = use_state(|| false);
    let role_editor_content = use_state(|| html!{});
    let should_update = use_state(|| false);
    let jobs: UseStateHandle<Vec<Job>> = use_state(|| vec![]);

    let notifier = use_notification::<CustomNotification>();
    
    let is_menu_enabled = use_atom::<ShowMenu>();
    is_menu_enabled.set(ShowMenu {value: true});

    {
        let jobs = jobs.clone();
        let should_update = should_update.clone();
        use_effect_with(*should_update, move |_| {
            should_update.set(false);
            spawn_local(async move {
                let request_jobs: Vec<Job> = get_jobs().await.unwrap_or(vec![]);
                jobs.set(request_jobs);
            })
        })
    }

    let onclose = {
        let show_role_editor = show_role_editor.clone();
        let role_editor_content = role_editor_content.clone();
        Callback::from(move |_| {
            role_editor_content.set(html!{<JobEditor/>});
            show_role_editor.set(false);
    })};

    let on_add_job = {
        let show_role_editor = show_role_editor.clone();
        let role_editor_content = role_editor_content.clone();
        let should_update = should_update.clone();
        let onclose = onclose.clone();
        Callback::from(move |_| {
            let should_update = should_update.clone();
            role_editor_content.set(html! {
                <JobEditor oncomplete={onclose.clone().reform(move |_| {
                    should_update.set(true);
                    }
                )
                }/>
            });
            show_role_editor.set(true);
        })
    };

    let on_edit_job = {
        let show_role_editor = show_role_editor.clone();
        let role_editor_content = role_editor_content.clone();
        let should_update = should_update.clone();
        let onclose = onclose.clone();
        let jobs = jobs.clone();
        Callback::from(move |index: usize| {
            let should_update = should_update.clone();
            role_editor_content.set(html! {
                <JobEditor job={jobs[index].clone()} oncomplete={onclose.clone().reform(move |_| {
                    should_update.set(true);
                    }
                )
                }/>
            });
            show_role_editor.set(true);
        })
    };

    let on_delete_job = {
        let should_update = should_update.clone();
        let role_editor_content = role_editor_content.clone();
        let show_role_editor = show_role_editor.clone();
        let jobs = jobs.clone();

        let notifier = notifier.clone();

        Callback::from(move |index: usize| {
            let should_update = should_update.clone();
            let jobs = jobs.clone();
            let show_role_editor = show_role_editor.clone();
            
            let notifier = notifier.clone();

            let on_cancel = {
                let show_role_editor = show_role_editor.clone();
                let role_editor_content = role_editor_content.clone();
                
                Callback::from(move |_| {
                    show_role_editor.set(false);
                    role_editor_content.set(html! {})
                })
            };

            let on_delete = {
                let jobs = jobs.clone();
                let should_update = should_update.clone();
                let show_role_editor = show_role_editor.clone(); 
                let role_editor_content = role_editor_content.clone();

                let notifier = notifier.clone();
                
                Callback::from(move |_| {
                    let jobs = jobs.clone();
                    let should_update = should_update.clone();
                    let show_role_editor = show_role_editor.clone();
                    let role_editor_content = role_editor_content.clone();

                    let notifier = notifier.clone();

                    spawn_local(async move {
                        match delete_job(jobs[index].id).await {
                            Ok(()) => {
                                should_update.set(true);
                                show_role_editor.set(false);
                                role_editor_content.set(html! {});
                            },
                            Err(message) => notifier.spawn(CustomNotification::new(&format!("Erro ao apagar vaga: {}", message)))
                        }
                        
                    })
                })
            };
            
            role_editor_content.set(html! {
                <div class="p-2">
                    <p class="text-center">{format!("Deseja realmente deletar a vaga '{}'?", jobs[index].title.clone())}</p>
                    <div class="flex">
                        <button onclick={on_cancel} class="flex-grow border border-amber-700 rounded-xl text-amber-700 py-2">{"Cancelar"}</button>
                        <button onclick={on_delete} class="flex-grow p-2 text-white rounded-xl mx-1 bg-red-500">{"Apagar"}</button>
                    </div>
                </div>
            });
            show_role_editor.set(true);
        })
    };

    html! {
        <div>
            <button onclick={Callback::from(|_| ())}></button>
            // mock a jobtable 
            <JobTable
                jobs={(*jobs).clone()} 
                {on_add_job}
                {on_edit_job}
                {on_delete_job}
                />
                if *show_role_editor {
                    <FloatWindow onclose={onclose.reform(|_| ())}>{(*role_editor_content).clone()}</FloatWindow>
                }
        </div>
    }
}
