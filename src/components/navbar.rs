use leptos::*;
use leptos_router::*;

use crate::functions::is_user_logged_in;
use crate::app::NavbarReloader;

#[component]
pub fn NavButton<H: ToHref + 'static>(#[prop(default = false)] is_mobile: bool, #[prop(default = "")] class: &'static str, children: Children, href: H) -> impl IntoView {
    let classes = move || if is_mobile {
        let mut default = "block py-4 px-4 text-sm transition easy-in-out hover:bg-red-300 ".to_owned();
        default.push_str(class);
        default
    } else {
        let mut default = "py-2 px-3 transition easy-in-out hover:bg-red-300 ".to_owned();
        default.push_str(class);
        default
    };

    view! {
        <A href class=classes()>
            {children()}
        </A>
    }
}

#[component]
pub fn Navbar(is_logged: Resource<bool, bool>) -> impl IntoView {
    let (menu_open, set_menu_open) = create_signal(false);
    let set_reload = use_context::<NavbarReloader>().unwrap().0;
    set_reload.update(|toggle: &mut bool| { *toggle = !*toggle; });

    view! {
        <nav class="bg-[#ef4444] text-white">
            <div class="xl:max-w-6xl xl:px-4 mx-auto">
                <div class="flex items-center justify-center">
                    <img class="object-contain h-40 w-40" src="/assets/logo/white.png"/>
                </div>
                <div class="flex justify-center">
                    <div class="flex space-x-4">
                        <div class="hidden md:flex md:items-center md:space-x-1">
                            <NavButton href="/">"Início"</NavButton>
                            <NavButton href="/vagas">"Vagas"</NavButton>
                            <Transition fallback=move || {
                                view! {}
                            }>

                                {move || {
                                    is_logged
                                        .get()
                                        .map(|log| match log {
                                            true => {
                                                view! {
                                                    <>
                                                        <NavButton href="/perfil" class="border-l">
                                                            "Meu Perfil"
                                                        </NavButton>
                                                    </>
                                                }
                                            }
                                            false => {
                                                view! {
                                                    <>
                                                        <NavButton href="/login" class="border-l">
                                                            "Login"
                                                        </NavButton>
                                                        // <NavButton href="/cadastrar">"Cadastrar"</NavButton>
                                                    </>
                                                }
                                            }
                                        })
                                }}

                            </Transition>
                        </div>
                    </div>
                </div>
            </div>
            <button
                class="md:hidden w-full block py-4 px-4 text-sm transition easy-in-out hover:bg-red-300"
                on:click=move |_| set_menu_open(!menu_open())
            >
                Menu
            </button>
            {move || match menu_open() {
                true => {
                    view! {
                        <div class="md:hidden transition-all bg-[#cf1f1f]">
                            <NavButton href="/" is_mobile=true>
                                "Início"
                            </NavButton>
                            <NavButton href="/vagas" is_mobile=true>
                                "Vagas"
                            </NavButton>
                            <Transition>
                                {move || {
                                    is_logged
                                        .get()
                                        .map(|log| match log {
                                            true => {
                                                view! {
                                                    <>
                                                        <NavButton
                                                            href="/perfil"
                                                            is_mobile=true
                                                            class="border-t"
                                                        >
                                                            "Meu Perfil"
                                                        </NavButton>
                                                    </>
                                                }
                                            }
                                            false => {
                                                view! {
                                                    <>
                                                        <NavButton href="/login" is_mobile=true class="border-t">
                                                            "Login"
                                                        </NavButton>
                                                        // <NavButton href="/cadastrar" is_mobile=true>
                                                        //     "Cadastrar"
                                                        // </NavButton>
                                                    </>
                                                }
                                            }
                                        })
                                }}

                            </Transition>
                        </div>
                    }
                }
                false => {
                    view! { <div></div> }
                }
            }}

        </nav>
    }
}