use yew::prelude::*;
use types::market::Market;
use web_sys::HtmlInputElement;
use crate::utils::console_log;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub markets: Vec<Market>,
    pub market_id: String,
    pub on_market_change: Callback<String>
}

#[function_component(MarketList)]
pub fn market_list(props: &Props) -> Html {
    let search_term = use_state(|| String::from(""));
    let markets = filter_markets(&props.markets, &search_term.to_string().to_uppercase());

    html! {
        <div class="h-full grid grid-rows-[min-content_1fr]">
            <div>
                <input 
                    oninput={move |e: InputEvent| {
                        let input: HtmlInputElement = e.target_unchecked_into();
                        let value = input.value();
                        search_term.set(value);
                    }} 
                    type="text"
                    class="px-4 py-2 w-full bg-slate-100 ring-blue-200 ring-inset outline-none"
                    placeholder="Search..."
                    autocomplete="off"
                /> 
            </div>
            <div class="overflow-auto">
                <ul>
                { for markets.iter().cloned().map(move |m| {
                    let on_change = props.on_market_change.clone();
                    let mut active_class = "";
                    if props.market_id == m.id {
                        active_class = "bg-slate-200"
                    }
                    let class_name = format!("w-full p-4 text-left focus:ring-2 ring-blue-200 ring-inset outline-none {}", active_class);

                    html! {
                        <li class="border-b border-slate-100">
                            <button
                                onclick={move |_: MouseEvent| {
                                    let id = m.id.clone();
                                    console_log(format!("selected {}", id));
                                    on_change.emit(id);
                                }}
                                class={class_name}
                                type="button"
                            >
                                <span class="block">{ &m.instrument_code }</span>
                                <span class="block text-xs text-slate-500">{format!("{} | {}", &m.state, &m.trading_mode)}</span>
                            </button>
                        </li>
                    }
                })}
                </ul> 
            </div>
        </div>
    }
}

fn filter_markets(markets: &Vec<Market>, term: &str) -> Vec<Market> {
    markets
        .iter()
        .cloned()
        .filter(|m| m.instrument_code.contains(&term))
        .collect::<Vec<_>>()
}


#[allow(unused_imports)] // for testing
use types::market::{State, TradingMode};

#[test]
fn test_filter_markets() {
    let test_markets = vec![
        Market {
            id: String::from("1"),
            decimal_places: 2,
            position_decimal_places: 2,
            instrument_code: String::from("a"),
            instrument_name: String::from("instrument-name"),
            state: State::Active,
            trading_mode: TradingMode::Continuous,
        },
        Market {
            id: String::from("2"),
            decimal_places: 2,
            position_decimal_places: 2,
            instrument_code: String::from("b"),
            instrument_name: String::from("instrument-name"),
            state: State::Active,
            trading_mode: TradingMode::Continuous,
        }
    ];
    assert_eq!(filter_markets(&test_markets, "c").len(), 0);
    assert_eq!(filter_markets(&test_markets, "a").len(), 1);
    assert_eq!(filter_markets(&test_markets, "b").len(), 1);
}