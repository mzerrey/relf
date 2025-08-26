use yew::prelude::*;
use yew_router::prelude::*;
use crate::frontend::services::router::Route;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class="container">
            <h1 class="title">{"Relf"}</h1>
            <div class="menu">
                <ul>
                    <li><Link<Route> to={Route::Outside}>{"Outside"}</Link<Route>></li>
                    <li><Link<Route> to={Route::Inside}>{"Inside"}</Link<Route>></li>
                    <li><Link<Route> to={Route::Data}>{"Data"}</Link<Route>></li>
                </ul>
            </div>
        </div>
    }
}