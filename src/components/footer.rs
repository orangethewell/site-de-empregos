use leptos::*;
use leptos_router::*;

#[component]
pub fn Footer() -> impl IntoView {
    
    view! {
        <footer class="text-white block md:flex p-4 justify-center bg-zinc-900">
            <ul>
                <li>"Início"</li>
                <li>"Vagas"</li>
            </ul>
            <ul>
                <li>"Instagram"</li>
                <li>"Whatsapp"</li>
            </ul>
        </footer>
    }
}