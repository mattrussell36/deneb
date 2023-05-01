mod markets;
mod market;

use crate::markets::Markets;
use crate::market::Market;
use yew::prelude::*;

#[function_component(Root)]
fn root() -> Html {
    let market = use_state(|| None);
    let on_market_change = {
        let market = market.clone();
        Callback::from(move |market_id: String| {
            market.set(Some(market_id))
        })
    };

    html! {
        <div class="w-screen h-screen grid grid-cols-[300px_1fr]">
            <div class="min-h-0 border-r border-slate-200">
                <Markets {on_market_change} />
            </div>
            <main class="min-h-0">
                <div class="p-4">
                    {match market.as_ref() {
                        Some(id) => html! {
                            <Market market_id={String::from(id)} />
                        },
                        None => html! {
                            <h1 class="text-xl text-center mt-20">{"No market selected"}</h1>
                        }
                    }}
                </div>
            </main>
        </div> 
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<Root>::new().render();
}