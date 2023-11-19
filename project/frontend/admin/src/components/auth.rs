use yew::{prelude::*, platform::spawn_local};
use yew::html::ChildrenRenderer;
use yew::virtual_dom::VChild;
use common::user_have_permission;
use yew_router::prelude::use_navigator;

use crate::app::Route;

#[derive(Properties, PartialEq)]
pub struct AuthFallbackProps {
    pub children: Html,
}

#[function_component(AuthFallback)]
pub fn auth_fallback(props: &AuthFallbackProps) -> Html {
    html! {
        <>
            {props.children.clone()}
        </>
    }
}

#[derive(Properties, PartialEq)]
pub struct AuthProtectedProps {
    pub children: Html,
}

#[function_component(AuthProtected)]
pub fn protected(props: &AuthProtectedProps) -> Html {
    html! {
        <>
            {props.children.clone()}
        </>
    }
}

#[derive(Clone, derive_more::From, PartialEq)]
pub enum AuthItem {
    AuthFallback(VChild<AuthFallback>),
    AuthProtected(VChild<AuthProtected>),
}

#[allow(clippy::from_over_into)]
impl Into<Html> for AuthItem {
    fn into(self) -> Html {
        match self {
            Self::AuthFallback(child) => child.into(),
            Self::AuthProtected(child) => child.into()
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct AuthGuardProps {
    pub children: ChildrenRenderer<AuthItem>,
    pub permission: AttrValue,
    #[prop_or_default]
    pub loading_unit: Html
}

#[function_component(AuthGuard)]
pub fn auth_guard(props: &AuthGuardProps) -> Html {
    let access_granted = use_state(|| String::from("await"));
    let navigator = use_navigator().unwrap();

    {
        let name = props.permission.clone();
        let access_granted = access_granted.clone();
        let navigator = navigator.clone();
        use_effect_with((), move |_| {
            let access_granted = access_granted.clone();
            spawn_local(async move {
                match user_have_permission(name.to_string()).await {
                    Ok(condition) => access_granted.set(if condition {"yes".to_owned()} else {"no".to_owned()}),
                    Err(err) => navigator.push(&Route::Login)
                }
            })
        })
    }

    html! {
        <>
        if *access_granted == "await" {
            {props.loading_unit.clone()}
        }
        { for props.children.iter().map(|child| match child {
                AuthItem::AuthProtected(child) => html! { {if *access_granted == "yes" { child.clone() } else { html_nested!{<AuthProtected>{""}</AuthProtected>}}}},
                AuthItem::AuthFallback(child) => html! { {if *access_granted == "no" { child.clone() } else { html_nested!{<AuthFallback>{""}</AuthFallback>}}} },
        })}
        </>
    }
}