use yew::prelude::*;

#[function_component(HomePage)]
pub fn home() -> Html {
    let percentage = use_state(|| 0);
    let onclick = {
        let percentage = percentage.clone();
        Callback::from(move |_| {
            percentage.set(*percentage + 1);
        })
    };

    html! {
        <main class="animate-drop-in">
            <div class="w-full bg-neutral-200 dark:bg-neutral-600">
                <div
                    class="bg-primary p-0.5 text-center text-xs font-medium leading-none text-primary-100 transition-all"
                    style={format!("width: {}%", *percentage)}>
                    {format!("{}%", *percentage)}
                </div>
            </div>
            <button {onclick} type="button" data-te-ripple-init="true" class="inline-block rounded bg-primary px-6 py-2.5 text-xs font-medium uppercase leading-tight text-white shadow-md transition duration-150 ease-in-out hover:bg-primary-700 hover:shadow-lg focus:bg-primary-700 focus:shadow-lg focus:outline-none focus:ring-0 active:bg-primary-800 active:shadow-lg">
            {"Button"}
            </button>
        </main>
    }
}

