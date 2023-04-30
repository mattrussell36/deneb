mod app;
mod markets;

use crate::app::App;
use crate::markets::Markets;
use yew::prelude::*;

#[function_component(Root)]
fn root() -> Html {
    html! {
        <div class="w-screen h-screen grid grid-cols-[300px_1fr]">
            <div class="min-h-0 border-r border-slate-200">
                <Markets />
            </div>
            <main class="min-h-0">
                <App />
            </main>
        </div> 
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<Root>::new().render();
}