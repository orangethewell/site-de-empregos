use bounce::{Atom, use_atom};
use common::{login_user, user_have_permission};
use gloo::console::log;
use yew::{prelude::*, platform::spawn_local};
use web_sys::HtmlInputElement;
use yew_notifications::use_notification;
use yew_router::prelude::use_navigator;
use crate::{components::{SubmitButton, TextInput, InputStatus, menu::ShowMenu, notifications::CustomNotification}, app::Route};

#[derive(Atom, PartialEq, Default)]
pub struct LoggedUser {
    pub id: String,
    pub username: String,
    pub email: String
}

#[function_component(Login)]
pub fn login() -> Html {
    let email_ref = use_node_ref();
    let password_ref = use_node_ref();

    let logged_user = use_atom::<LoggedUser>();

    // Status Handling

    let is_loading = use_state(|| false);
    let email_status = use_state(|| InputStatus::Neutral);
    let password_status = use_state(|| InputStatus::Neutral);

    let navigator = use_navigator().unwrap();
    let notifier = use_notification::<CustomNotification>();
    let is_menu_enabled = use_atom::<ShowMenu>();
    is_menu_enabled.set(ShowMenu {value: false});

    let onsubmit = {
        let email_ref = email_ref.clone();
        let password_ref = password_ref.clone();
        let logged_user = logged_user.clone();

        let is_loading = is_loading.clone();
        let notifier = notifier.clone();
        let navigator = navigator.clone();

        let email_status = email_status.clone();
        let password_status = password_status.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let email = email_ref.cast::<HtmlInputElement>().unwrap().value();
            let password = password_ref.cast::<HtmlInputElement>().unwrap().value();
            let logged_user = logged_user.clone();

            let is_loading = is_loading.clone();
            let notifier = notifier.clone();
            let navigator = navigator.clone();

            let email_status = email_status.clone();
            let password_status = password_status.clone();
            
            spawn_local(async move {
                is_loading.set(true);
                let auth = login_user(email, password).await;
                match auth {
                    Ok(user) => {
                        logged_user.set(LoggedUser { id: user.id, username: user.username, email: user.email });
                        email_status.set(InputStatus::Success);
                        password_status.set(InputStatus::Success);
                        navigator.push(&Route::Dashboard);
                    },
                    Err(err) => {
                        email_status.set(InputStatus::Error);
                        password_status.set(InputStatus::Error);

                        notifier.spawn(CustomNotification::new(&format!("Erro na tentativa de fazer login: {}", err)));

                        is_loading.set(false);
                    }
                }
            });
        })
    };

    html! {
        <>
            <div class="flex justify-center items-center h-screen">
                <form {onsubmit}>
                    <div class="flex justify-center w-full"><img class="h-40 w-auto" src="admin/assets/logo_black.png"/></div>
                    <TextInput id="emailInput" status={(*email_status).clone()} type_handler="text" input_ref={email_ref}>{"Endereço de E-mail"}</TextInput>
                    <TextInput id="passwordInput" status={(*password_status).clone()} type_handler="password" input_ref={password_ref}>{"Senha"}</TextInput>
                
                    <SubmitButton is_loading={*is_loading}>{"Login"}</SubmitButton>
                </form>
            </div>
        </>
    }
}
