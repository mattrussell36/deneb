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
        <ContextProvider<Market> context={props.market.clone()}>
            <div>
                <div class="p-4 border-b border-slate-200">
                    <h1 class="text-lg">{instrument.code}</h1> 
                    <p class="text-sm text-slate-600 mb-1">{instrument.name}</p>
                    <div class="flex gap-1">
                        <Pill>{"Status: "}{props.market.state.clone()}</Pill> 
                        <Pill>{"Trading mode: "}{props.market.trading_mode.clone()}</Pill> 
                    </div>
                </div>
                <div class="p-4">
                    <Info /> 
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
            <h1>{market.tradable_instrument.instrument.code}</h1>
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