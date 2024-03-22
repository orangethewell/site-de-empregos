use leptos::logging::log;
use leptos::*;
use leptos_icons::*;
use icondata as i;
use leptos_meta::*;
use leptos_router::*;

use crate::functions::{get_job, JobModel};

#[derive(Params, PartialEq)]
struct JobInfoQuery {
    id: i32
}

/// Renders the home page of your application.
#[component]
pub fn JobInfo() -> impl IntoView {
    let query = use_query::<JobInfoQuery>();

    let id = move || {
        query.with(|query| {
            query.as_ref()
                .map(|query| query.id)
                .unwrap_or(-1)
        })
    };
    let job = create_resource(id, |id| async move {get_job(id).await.unwrap()});

    let job_position = move || {
        job.get().unwrap_or_default().position
    };

    let job_company = move || {
        job.get().unwrap_or_default().company
    };

    let job_requirements = move || {
        job.get().unwrap_or_default().requirements
    };

    let job_description = move || {
        job.get().unwrap_or_default().description
    };

    let any_job_description = move || {
        match job_description() {
            Some(description) => view! {
                <p class="mx-8 whitespace-pre-wrap">
                    {description}
                </p>
            }.into_view(),
            None => view! {
                <div class="flex justify-center content-center py-20">
                    <p class="text-gray-500 font-bold text-lg">"Nenhuma informação adicional."</p>
                </div>
            }.into_view()
        }
    };

    view! {
        <Title text="Informações da Vaga"/>
        <main class="flex justify-center w-full bg-slate-200 min-h-screen">
            <Transition>
                <div class="mx-2 text-left md:w-1/2 my-4 w-full self-start bg-white shadow-md">
                    <h1 class="mx-2 text-2xl font-bold my-1">{job_position}</h1>
                    <hr class="mx-3"/>
                    <div class="mx-3">
                        <h2 class="text-gray-500 font-bold my-2 text-lg">"Requisitos"</h2>
                        <div class="mx-8">
                            <ul class="list-disc">
                                <For
                                    each=job_requirements
                                    key=move |requirement| {job_requirements().iter().position(|el| el == requirement).unwrap()}
                                    let:requirement
                                >
                                    <li>{requirement}</li>
                                </For>
                            </ul>
                        </div>
                        <h2 class="text-gray-500 font-bold my-2 text-lg">"Descrição"</h2>
                        {any_job_description}
                        <p class="text-right text-gray-500 my-2">"Vaga publicada por "<b class="font-bold">{job_company}</b></p>
                        <button
                            class="rounded-md hover:shadow-[0_2px_8px_0_rgba(0,0,0,0.5)] transition-all bg-[#ef4444] hover:bg-[#d83c3c] text-white p-2 w-full mb-5"
                        >
                            "Tenho interesse"
                        </button>
                    </div>
                </div>
            </Transition>
        </main>
    }
}