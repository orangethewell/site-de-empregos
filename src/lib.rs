pub mod components;
pub mod pages;
pub mod functions;

pub mod app;

#[cfg(feature = "ssr")]
use sea_orm::{DatabaseConnection, Database};

#[cfg(feature = "ssr")]
use lettre::{AsyncSmtpTransport, Tokio1Executor};

#[cfg(feature = "ssr")]
use handlebars::Handlebars;

#[cfg(feature = "ssr")]
#[derive(Clone)]
pub struct AppState<'a> {
    pub conn: DatabaseConnection,
    pub secret_key: String,
    pub mailer: AsyncSmtpTransport<Tokio1Executor>,
    pub template_engine: Handlebars<'a>,
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::*;
    use leptos::*;

    console_error_panic_hook::set_once();

    mount_to_body(App);
}
