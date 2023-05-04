use yew::prelude::*;
use types::market::{Market};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub market: Market,
}

#[function_component(Trade)]
pub fn trade(props: &Props) -> Html {
    let market_context = props.market.clone();
    let market = &props.market;
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