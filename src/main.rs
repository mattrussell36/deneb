mod app;
mod markets;

use crate::app::App;
use crate::markets::Markets;
use yew::prelude::*;

#[function_component(Root)]
fn root() -> Html {
    html! {
        <div>
            <h1>{ "Rust Console" }</h1>
            <App />
            <Markets />
        </div> 
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<Root>::new().render();
}