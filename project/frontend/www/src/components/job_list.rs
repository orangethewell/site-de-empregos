use common::Job;
use yew::prelude::*;
use yew_icons::{IconId, Icon};

#[derive(Properties, PartialEq)]
pub struct JobListProps {
    pub jobs: Vec<Job>,
    pub onselect: Callback<usize>
}

fn render_job(job: &Job, index: usize, onselect: Callback<usize>) -> Html {
    let onselect = onselect.reform(move |_| index);

    html! {
        <div onclick={onselect} class="mb-4 p-4 bg-white shadow-lg">
            <div class="flex mt-2">
                <div class="flex items-center justify-center px-4">
                <Icon icon_id={IconId::BootstrapBriefcase} class="flex-shrink-0 text-lg"/>
                </div>
                <div class="flex-grow">
                    <h2 class="text-xl font-semibold">{&job.title}</h2>
                    <p>{&job.branch}</p>
                </div>
            </div>
        </div>
    }
}

#[function_component(JobList)]
pub fn job_list(props: &JobListProps) -> Html {
    html! {
        <>
            {for props.jobs.iter().enumerate().map(|(index, job)| render_job(job, index, props.onselect.clone()))}
        </>
    }
}