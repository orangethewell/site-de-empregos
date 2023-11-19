use bounce::use_atom;
use common::get_jobs_count;
use yew::{prelude::*, platform::spawn_local};

use crate::{components::menu::ShowMenu, pages::login::LoggedUser};

#[function_component(Dashboard)]
pub fn dashboard() -> Html {
    let is_menu_enabled = use_atom::<ShowMenu>();
    is_menu_enabled.set(ShowMenu {value: true});

    let num_of_jobs = use_state(|| 0);
    let logged_user = use_atom::<LoggedUser>();

    {
        let num_of_jobs = num_of_jobs.clone();
        use_effect_with((), move |_| {
            let num_of_jobs = num_of_jobs.clone();
            spawn_local(async move {
                num_of_jobs.set(get_jobs_count().await)
            })
        })
    }

    html! {
        <div class="animate-{slideIn_1s_ease-in-out_1}">
            <h1 class="text-4xl mb-8 font-bold">{"Olá, "}{logged_user.username.clone()}{"!"}</h1>
            <div class="flex flex-col md:items-start md:flex-row items-stretch gap-2">
                <div class="flex-grow p-2 border rounded-md border-red-950">
                    <p class="text-center mb-2 font-bold text-2xl">{"Vagas de Emprego"}</p>
                    <p>{"Vagas Disponíveis: "}<b>{*num_of_jobs}</b></p>
                </div>
                <div class="flex-grow border rounded-md border-red-950">{"item two"}</div>
                <div class="flex-grow border rounded-md border-red-950">{"item three"}</div>
            </div>
        </div>
    }
}