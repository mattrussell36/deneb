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
                <p class="text-sm text-slate-600">{instrument.name}</p>
            </div>
            <div class="p-4">{"Content"}</div>
        </div>
    }
}