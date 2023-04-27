mod app;

use crate::app::App;
use yew::prelude::*;

#[function_component(Root)]
fn root() -> Html {
    html! {
        <div>
            <h1>{ "Rust Console" }</h1>
            <App />
        </div> 
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<Root>::new().render();
}