use yew::prelude::*;
use yew_router::prelude::*;
use crate::{pages, components::{Navbar, Footer}};

#[derive(Clone, Routable, PartialEq, Debug, serde::Serialize)]
pub enum Route {
    #[at("/")]
    Home,

    #[at["/vagas"]]
    Jobs,

    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => {
            html! {<pages::HomePage/>}
        }

        Route::Jobs  => {
            html!{<pages::JobsPage/>}
        }
        _ => {
            html!{<pages::handlers::NotFoundPage/>}
        }
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Navbar/>
            <main>
                <Switch<Route> render={switch}/>
            </main>
            <Footer/>
            <script src="https://cdn.jsdelivr.net/npm/tw-elements/dist/js/tw-elements.umd.min.js"></script>
        </BrowserRouter>
    }
}
