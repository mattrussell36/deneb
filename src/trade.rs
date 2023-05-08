use serde::{Serialize, Deserialize};
use yew::{prelude::*, platform::spawn_local};
use types::market::{Market, EventPayload};
use tauri_sys::{tauri, event};
use wasm_bindgen::prelude::*;
use futures::stream::StreamExt;

use crate::utils::console_log;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "event"])]
    async fn listen(event: &str, closure: &Closure<dyn FnMut()>); 
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub market: Market,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
struct EventArgs {
    num: u16,
}

#[function_component(Trade)]
pub fn trade(props: &Props) -> Html {
    let market_context = props.market.clone();
    let market = &props.market;

    use_effect_with_deps(move |_| {
        spawn_local(async move {
            let mut events = event::listen::<EventPayload>("my-event")
                .await
                .expect("failed to listen");


            while let Some(event) = events.next().await {
                console_log(format!("trade.rs {:#?}", event));
            }
        });

        return || {}
    }, ());

    let onclick = Callback::from(|_: MouseEvent| {
        spawn_local(async move {
            tauri::invoke::<_, ()>("emit_event", &EventArgs { num: 1 })
                .await
                .unwrap();
        });
    });

    html! { 
        <ContextProvider<Market> context={market_context}>
            <div>
                <div class="p-4 border-b border-slate-200">
                    <h1 class="text-lg">{&market.instrument_code}</h1> 
                    <p class="text-sm text-slate-600 mb-1">{&market.instrument_name}</p>
                    <div class="flex gap-1">
                        <Pill>{"Status: "}{&market.state}</Pill> 
                        <Pill>{"Trading mode: "}{&market.trading_mode}</Pill> 
                    </div>
                </div>
                <div class="p-4">
                    <Info /> 
                    <button {onclick}>{"Test"}</button>
                </div>
            </div>
        </ContextProvider<Market>>
    }
}

#[derive(Properties, PartialEq)]
pub struct PillProps {
    pub children: Children,
}

#[function_component]
pub fn Pill(props: &PillProps) -> Html {
    html! { 
        <div class="bg-slate-300 rounded px-1 py-0.5 text-xs">
            { for props.children.iter() }
        </div>
    }
}

#[function_component]
pub fn Info() -> Html {
    let market = use_market();

    html! { 
        <div class="bg-slate-300 rounded py-2 px-4">
            <h1>{market.instrument_code}</h1>
        </div>
    }
}

#[hook]
fn use_market() -> Market {
    let market = use_context::<Market>();

    match market {
        Some(data) => data,
        None => {
            panic!("use_market must be used inside a market context");
        }
    }
}