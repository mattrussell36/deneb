use yew::prelude::*;
use types::market::Market;
use web_sys::HtmlInputElement;
use crate::utils::console_log;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub markets: Vec<Market>,
    pub on_market_change: Callback<String>
}

#[function_component]
pub fn MarketList(props: &Props) -> Html {
    let search_term = use_state(|| String::from(""));
    let m = props.markets.clone();
    let markets = filter_markets(&m, &search_term.to_string().to_uppercase());

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
                    class="px-4 py-2 w-full outline-none bg-slate-100 focus:bg-slate-300"
                    placeholder="Search..."
                    autocomplete="off"
                /> 
            </div>
            <div class="overflow-auto">
                <ul>
                { for markets.iter().cloned().map(|m| {
                    let on_change = props.on_market_change.clone();
                    html! {
                        <li class="border-b border-slate-100">
                            <button
                                onclick={move |_: MouseEvent| {
                                    let id = m.id.clone();
                                    console_log(format!("selected {}", id));
                                    on_change.emit(id);
                                }}
                                class="w-full p-4 text-left enabled:focus:bg-slate-300 outline-none"
                                type="button"
                            >
                                { &m.tradable_instrument.instrument.code }
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
        .filter(|m| m.tradable_instrument.instrument.code.contains(&term))
        .collect::<Vec<_>>()
}


#[allow(unused_imports)] // for testing
use types::market::{TradableInstrument, Instrument};

#[test]
fn test_filter_markets() {
    let test_markets = vec![
        Market {
            id: String::from("1"),
            decimal_places: 2,
            position_decimal_places: 2,
            tradable_instrument: TradableInstrument {
                instrument: Instrument {
                    id: String::from("instrument-id"),
                    code: String::from("a"),
                    name: String::from("instrument-name")
                }
            }
        },
        Market {
            id: String::from("2"),
            decimal_places: 2,
            position_decimal_places: 2,
            tradable_instrument: TradableInstrument {
                instrument: Instrument {
                    id: String::from("instrument-id"),
                    code: String::from("b"),
                    name: String::from("instrument-name")
                }
            }
        }
    ];
    assert_eq!(filter_markets(&test_markets, "c").len(), 0);
    assert_eq!(filter_markets(&test_markets, "a").len(), 1);
    assert_eq!(filter_markets(&test_markets, "b").len(), 1);
}