use leptos::*;
use leptos_meta::*;

use crate::components::{Navbar, Footer};

/// Renders the home page of your application.
#[component]
pub fn Register() -> impl IntoView {
    view! {
        <Title text="Cadastro"/>
        <main class="flex justify-center w-full bg-slate-200 min-h-screen">
            <div class="mx-2 md:w-1/2 my-4 w-full self-start bg-white shadow-md">
                <form class="p-4">
                    <h1 class="text-3xl font-bold mb-2">"Cadastrar"</h1>
                    <hr class="mx-4 mb-6 h-[2px] bg-gray-300"/>
                    <div class="mb-4">
                        <p class="text-left font-bold">"Nome completo"</p>
                        <input id="username" class="rounded-md border-2 w-full border-gray-400 flex-grow p-2 focus:outline-none focus:border-gray-600" type="username"/>
                    </div>
                    <div class="mb-4">
                        <p class="text-left font-bold">"Email"</p>
                        <input id="email" class="rounded-md border-2 w-full border-gray-400 flex-grow p-2 focus:outline-none focus:border-gray-600" type="email"/>
                    </div>
                    <div class="mb-4">
                        <p class="text-left font-bold">"Senha"</p>
                        <input id="password" class="rounded-md border-2 w-full border-gray-400 flex-grow p-2 focus:outline-none focus:border-gray-600" type="password"/>
                    </div>
                    <p class="w-full text-start mb-4 flex gap-2"><input class="appearance-none w-4 h-4 peer relative border-2 transition-colors duration-75 border-gray-500 mt-1 rounded-sm shrink-0 bg-white checked:bg-red-500 checked:border-0" type="checkbox"/>" Li e estou de acordo com os "<a>"Termos de Uso."</a><svg
                    class="absolute 
                      w-4 h-4 mt-1
                      hidden peer-checked:block
                      pointer-events-none
                      text-white"
                    xmlns="http://www.w3.org/2000/svg"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="3"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                  >
                    <polyline points="20 6 9 17 4 12"></polyline>
                  </svg></p>
                    <button class="rounded-md hover:shadow-[0_2px_8px_0_rgba(0,0,0,0.5)] transition-all bg-[#ef4444] hover:bg-[#d83c3c] text-white p-2 w-full mb-5">"Criar cadastro"</button>
                </form>
            </div>
        </main>
    }
}