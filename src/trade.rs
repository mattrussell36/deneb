use yew::prelude::*;
use types::market::{Market};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub market: Market,
}

#[function_component]
pub fn Trade(props: &Props) -> Html {
    let tradable_instrument = props.market.tradable_instrument.clone();
    let instrument = tradable_instrument.instrument.clone();
    html! { 
        <div>
            <div class="p-4 border-b border-slate-200">
                <h1 class="text-lg">{instrument.code}</h1> 
                <p class="text-sm text-slate-600 mb-1">{instrument.name}</p>
                <div class="flex gap-1">
                    <Pill>{"Status: "}{props.market.state.clone()}</Pill> 
                    <Pill>{"Trading mode: "}{props.market.trading_mode.clone()}</Pill> 
                </div>
            </div>
            <div class="p-4">{"Content"}</div>
        </div>
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