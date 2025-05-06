use std::ops::Div;

use api::fetch_all;
use models::Task;
use yew::prelude::*;

mod api;
mod components;
mod models;

#[function_component(App)]
fn app() -> Html {
    wasm_bindgen_futures::spawn_local(async {
        let tasks = fetch_all().await.unwrap();
    });

    html!(
        <div>
            <h1>{"text"}</h1>
        </div>
    )
}

fn main() {
    yew::Renderer::<App>::new().render();
}
