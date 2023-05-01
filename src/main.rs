
mod utils;
mod app_init;
mod trade;
mod market_list;

use crate::app_init::AppInit;
use yew::prelude::*;

#[function_component(Root)]
fn root() -> Html {
    html! {
       <AppInit />
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<Root>::new().render();
}