use wasm_bindgen::prelude::*;

// Only include models needed for frontend
pub mod models {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct Outside {
        pub uuid: String,
        pub name: String,
        pub context: String,
        pub url: String,
        pub percentage: Option<i32>,
    }

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct Inside {
        pub uuid: String,
        pub context: String,
        pub date: String,
    }
}

mod frontend;

use frontend::app::App;

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<App>::new().render();
}