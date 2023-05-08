use serde::{Serialize, Deserialize};
use yew::{prelude::*, platform::spawn_local};
use types::market::{Market, SubscribeToMarketArgs, MarketData};
use tauri_sys::{tauri, event};
use futures::stream::StreamExt; // this is needed in scope for stream to work

use crate::utils::{format_dp, console_log}; 

#[derive(Properties, PartialEq)]
pub struct Props {
    pub market: Market,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
struct EventArgs {
    num: u16,
}

#[derive(Clone)]
struct MarketDataState {
    mark_price: String,
    best_bid_price: String,
    best_bid_volume: String,
    best_offer_price: String,
    best_offer_volume: String,
    mid_price: String
}

#[function_component(MarketView)]
pub fn market_view(props: &Props) -> Html {
    let market_context = props.market.clone();
    let market = &props.market;
    let id = market.id.clone();
    let id2= market.id.clone();
    let dep = id.clone();
    let default_state = MarketDataState {
        mark_price: "-".to_string(),
        best_bid_price: "-".to_string(),
        best_bid_volume: "-".to_string(),
        best_offer_price: "-".to_string(),
        best_offer_volume: "-".to_string(),
        mid_price: "-".to_string()
    };
    let reset_state = default_state.clone();

    let data_state = use_state(|| default_state);
    let data_state_clone = data_state.clone();
    let data_state_cleanup = data_state.clone();

    // listen for market data
    use_effect_with_deps(move |_| {
        spawn_local(async move {
            let mut events = event::listen::<MarketData>("market-data")
                .await
                .expect("failed to listen");


            while let Some(event) = events.next().await {
                data_state_clone.set(MarketDataState {
                    mark_price: event.payload.mark_price,
                    best_bid_price: event.payload.best_bid_price,
                    best_bid_volume: event.payload.best_bid_volume.to_string(),
                    best_offer_price: event.payload.best_offer_price,
                    best_offer_volume: event.payload.best_offer_volume.to_string(),
                    mid_price: event.payload.mid_price 
                });
            }
        });
        return || {}
    }, ());

    // start subscription to market data
    use_effect_with_deps(move |_| { 
        spawn_local(async move {
            let res = tauri::invoke("subscribe_to_market", &SubscribeToMarketArgs { id: String::from(id) })
                .await
                .expect("failed to subscribe");
            console_log(res);
        });

        return move || {
            data_state_cleanup.set(reset_state);
            spawn_local(async move {
                let res2 = tauri::invoke("unsubscribe_to_market", &SubscribeToMarketArgs { id: id2 })
                    .await
                    .expect("failed to unsubscribe");
                console_log(res2);
            });
        };
    }, dep);

    html! { 
        <ContextProvider<Market> context={market_context}>
            <div>
                <div class="p-4 border-b border-slate-200">
                    <div class="flex gap-2 items-end justify-between">
                        <header>
                            <h1 class="text-lg">{&market.instrument_code}</h1> 
                            <p class="text-sm text-slate-600">{&market.instrument_name}</p>
                        </header>
                        <div class="flex gap-2">
                            <StatItem
                                label={"Mark price"}
                                value={render_value(&data_state.mark_price, market.decimal_places as usize)}
                            />
                            <StatItem
                                label={"Best bid price"}
                                value={render_value(&data_state.best_bid_price, market.decimal_places as usize)}
                            />
                            <StatItem
                                label={"Best bid volume"}
                                value={render_value(&data_state.best_bid_volume.to_string(), market.position_decimal_places as usize)}
                            />
                            <StatItem
                                label={"Best offer price"}
                                value={render_value(&data_state.best_offer_price, market.decimal_places as usize)}
                            />
                            <StatItem
                                label={"Best offer volume"}
                                value={render_value(&data_state.best_offer_volume.to_string(), market.position_decimal_places as usize)}
                            />
                            <StatItem
                                label={"Mid price"}
                                value={render_value(&data_state.mid_price, market.decimal_places as usize)}
                            />
                        </div>
                    </div>
                </div>
                <div class="p-4">
                    <div class="flex gap-2">
                        <Pill>{"Status: "}{&market.state}</Pill> 
                        <Pill>{"Trading mode: "}{&market.trading_mode}</Pill> 
                    </div>
                </div>
            </div>
        </ContextProvider<Market>>
    }
}

fn render_value(value: &str, dps: usize) -> String {
    if value == "-" {
        return "-".to_string();
    }

    format_dp(value, dps)
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

#[derive(Properties, PartialEq)]
pub struct StatItemProps {
    pub label: String,
    pub value: String,
}

#[function_component]
pub fn StatItem(props: &StatItemProps) -> Html {
    html! { 
        <div class="text-xs">
            <div class="text-slate-500">{&props.label}</div>
            <div class="">{&props.value}</div>
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