use common::{Job, get_jobs};
use gloo::console::log;
use yew::{prelude::*, platform::spawn_local};
use crate::components::JobList;

#[function_component(JobsPage)]
pub fn jobs() -> Html {
    let jobs = use_state(||Vec::<Job>::new());
    
    {
        let jobs = jobs.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                let request_jobs: Vec<Job> = get_jobs().await.unwrap_or(vec![]);
                jobs.set(request_jobs);
            })
        })
    }

    let colapse = use_state(|| true);

    let selected_job = use_state(|| Job::default());
    
    let onselect = {
        let colapse = colapse.clone();
        let selected_job = selected_job.clone();
        let jobs = jobs.clone();
        Callback::from(move |index: usize| {
            colapse.set(false);
            selected_job.set(jobs[index].clone());
        })
    };

    let oncancel = {
        let colapse = colapse.clone();
        Callback::from(move |_| {
            colapse.set(true)
        })
    };

    let oninterested = {
        let selected_job = selected_job.clone();
        Callback::from(move |_| {
            let _ = gloo::utils::window().location().set_href(
                &format!("https://api.whatsapp.com/send?phone=553498013642&text=Olá! Estou interessado na vaga de {} oferecido por {}", &selected_job.title, &selected_job.company)
            );      
        })
    };

    html! {
        <main class="animate-drop-in">
        <div class="relative flex">
            <div class="flex-1 flex-grow min-h-screen sm:max-w-xs lg:max-w-md px-4 overflow-y-auto">
                <JobList {onselect} jobs={(*jobs).clone()}/>
            </div>
            <div class={classes!("flex-1", "p-4", "min-h-screen", "max-sm:absolute", "max-sm:left-0", "max-sm:right-0", "max-s:top-0", "max-sm:bottom-0", "bg-gray-100", "transition-transform", "transform", colapse.then(||"max-sm:translate-x-full"))}>
                if selected_job.title != "" {
                    <div class="flex align-top">
                        <h2 class="flex-grow text-xl font-semibold">{selected_job.title.clone()}</h2>
                        <p class="text-gray-400">{selected_job.branch.clone()}</p>
                    </div>
                    <p class="text-gray-400">{"Oferecido por "}<b>{selected_job.company.clone()}</b></p>
                    <div class="mt-8">
                        <p class="text-lg font-semibold">{"Requisitos"}</p>
                        <ul class="my-4">
                        {selected_job.requirements.iter().map(|requirement| html!{
                            <li class="list-disc ml-2 mb-1">{requirement.clone()}</li>
                        }).collect::<Html>()}
                        </ul>
                        <p class="text-lg font-semibold">{"Atividades"}</p>
                        <ul class="my-4">
                        {selected_job.activities.iter().map(|activity| html!{
                            <li class="list-disc ml-2 mb-1">{activity.clone()}</li>
                        }).collect::<Html>()}
                        </ul>
                    </div>
                    <div class="flex md:items-center absolute bottom-2 max-sm:flex-col gap-1 max-sm:left-2 right-2">
                        <button onclick={oncancel} data-te-ripple-init="true" data-te-ripple-color="#fca5a5" class="py-4 px-8 md:hidden bg-white border-2 border-red-600 text-red-600 hover:border-red-400 hover:text-red-400 transition flex-grow md:flex-grow-0">{"Cancelar"}</button>
                        <button onclick={oninterested} data-te-ripple-init="true" data-te-ripple-color="#fca5a5" class="py-4 px-8 bg-red-600 text-white hover:bg-red-400 transition flex-grow md:flex-grow-0">{"Tenho interesse"}</button>
                    </div>
                } else {
                    <div class="flex items-center justify-center">
                        <p class="text-gray-400 text-xl">{"Selecione uma vaga para ter mais informações"}</p>
                    </div>
                }
            </div>
        </div>
        </main>
    }
}

