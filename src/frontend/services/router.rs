use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/relf")]
    Relf,
    #[at("/outside")]
    Outside,
    #[at("/inside")]
    Inside,
    #[at("/data")]
    Data,
    #[not_found]
    #[at("/404")]
    NotFound,
}