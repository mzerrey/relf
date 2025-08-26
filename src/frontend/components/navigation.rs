use yew::prelude::*;
use yew_router::prelude::*;
use crate::frontend::services::router::Route;

#[derive(Properties, PartialEq)]
pub struct NavigationProps {
    pub title: String,
}

#[function_component(Navigation)]
pub fn navigation(_props: &NavigationProps) -> Html {
    let mobile_nav_open = use_state(|| false);
    let navigator = use_navigator().unwrap();
    let route = use_route::<Route>().unwrap_or(Route::Outside);
    
    let toggle_mobile_nav = {
        let mobile_nav_open = mobile_nav_open.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            mobile_nav_open.set(!*mobile_nav_open);
        })
    };

    let close_mobile_nav = {
        let mobile_nav_open = mobile_nav_open.clone();
        Callback::from(move |_| {
            mobile_nav_open.set(false);
        })
    };

    let go_home = {
        let navigator = navigator.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            navigator.push(&Route::Relf);
        })
    };

    html! {
        <>
            <div class={classes!("mobile-nav", (*mobile_nav_open).then_some("active"))}>
                <ul>
                    <li onclick={close_mobile_nav.clone()}>
                        <Link<Route> to={Route::Outside}>{"OUTSIDE"}</Link<Route>>
                    </li>
                    <li onclick={close_mobile_nav.clone()}>
                        <Link<Route> to={Route::Inside}>{"INSIDE"}</Link<Route>>
                    </li>
                    <li onclick={close_mobile_nav.clone()}>
                        <Link<Route> to={Route::Data}>{"DATA"}</Link<Route>>
                    </li>
                </ul>
            </div>
            
            <div class={classes!("overlay", (*mobile_nav_open).then_some("active"))} onclick={close_mobile_nav}></div>
            
            <nav>
                <div class={classes!("hamburger-menu", (*mobile_nav_open).then_some("active"))} onclick={toggle_mobile_nav.clone()}>
                    <span></span>
                    <span></span>
                    <span></span>
                </div>
                <div class="nav-title logo-clickable" onclick={go_home}>{"Relf"}</div>
                <ul>
                    <li class={if matches!(route, Route::Outside) { "active" } else { "" }}>
                        <Link<Route> to={Route::Outside}>{"OUTSIDE"}</Link<Route>>
                    </li>
                    <li class={if matches!(route, Route::Inside) { "active" } else { "" }}>
                        <Link<Route> to={Route::Inside}>{"INSIDE"}</Link<Route>>
                    </li>
                    <li class={if matches!(route, Route::Data) { "active" } else { "" }}>
                        <Link<Route> to={Route::Data}>{"DATA"}</Link<Route>>
                    </li>
                </ul>
            </nav>
        </>
    }
}