use yew::prelude::*;
use yew_router::prelude::*;
use crate::frontend::services::router::Route;
use web_sys::window;

#[function_component(Home)]
pub fn home() -> Html {
    use_effect_with((), |_| {
        if let Some(window) = window() {
            if let Some(document) = window.document() {
                if let Some(body) = document.body() {
                    let current_class = body.class_name();
                    if !current_class.contains("homepage") {
                        body.set_class_name(&format!("{} homepage", current_class));
                    }
                }
            }
        }
        
        || {
            if let Some(window) = window() {
                if let Some(document) = window.document() {
                    if let Some(body) = document.body() {
                        let current_class = body.class_name();
                        let new_class = current_class.replace("homepage", "").trim().to_string();
                        body.set_class_name(&new_class);
                    }
                }
            }
        }
    });

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