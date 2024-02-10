use leptos::*;
use leptos_meta::*;
use leptos_router::{use_params, IntoParam, Params, A};

use crate::components::{Navbar, Footer};
use crate::functions::{add_job, get_jobs_paginated, JobModel};

#[derive(Params, PartialEq, Clone, Debug)]
pub struct JobsParams {
    page_id: u64,
}

/// Renders the home page of your application.
#[component]
pub fn JobsPage() -> impl IntoView {
    let params = use_params::<JobsParams>();

    let jobs = create_resource(move || params().map(|params| params.page_id).ok().unwrap_or(0u64) , get_jobs_paginated);
    
    view! {
        <Title text="Vagas"/>
        <main class="flex justify-center w-full bg-slate-200 min-h-screen">
            <div class="mx-2 md:w-1/2 my-4 w-full self-start bg-white shadow-md">
                <div class="border-2 rounded-md my-6 max-w-16 mx-3 shadow-md flex">
                    <input class="flex-grow px-2" type="search"/>
                    <button class="p-2 border-l-2">"Pesquisar"</button>
                </div>
                <div>
                    <Transition fallback=move || view! {
                        <p>"Carregando..."</p>
                    }>
                        {move || {
                            jobs.get().map(|data| match data {
                                Ok(jobs_page) => {
                                    view! {
                                        <ul>
                                            {
                                                jobs_page.current_content.iter()
                                                    .map(|job_model| {
                                                        view! {
                                                        <li>
                                                            <div class="rounded-md border-2 m-4">
                                                                <h2 class="font-bold text-left">{&job_model.position}</h2>
                                                            </div>
                                                        </li>
                                                        }
                                                    })
                                                    .collect_view()
                                            }
                                        </ul>
                                        <div class="my-4">{move || {
                                            (0..jobs_page.num_pages).map(|num| {
                                                view! {
                                                    <Show 
                                                        when=move || {params().map(|params| params.page_id).ok().unwrap_or(0u64) == num + 1}
                                                        fallback=move || view! {<A href=format!("/vagas/{}", num + 1) class="px-3 py-2 m-2 border-2 rounded-md shadow-md">{num + 1}</A>}
                                                    >
                                                    <A href="#" class="px-3 py-2 m-2 border-2 rounded-md shadow-md border-red-500">{num + 1}</A>
                                                    </Show>
                                                }
                                            }).collect_view()
                                        }}
                                        </div>
                                    }.into_view()
                                },

                                _ => view! { <pre>"Error"</pre> }.into_view(),
                              })
                        }}
                    </Transition>
                </div>
            </div>
        </main>
    }
}