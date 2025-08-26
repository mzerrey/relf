use yew::prelude::*;
use yew_router::prelude::*;
use crate::frontend::services::router::Route;
use crate::frontend::pages::{
    home::Home,
    inside::InsidePage,
    outside::OutsidePage,
    data::Data,
};

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Relf => html! { <Home /> },
        Route::Outside => html! { <OutsidePage /> },
        Route::Inside => html! { <InsidePage /> },
        Route::Data => html! { <Data /> },
        Route::NotFound => html! { <h1>{"Page Not Found"}</h1> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}