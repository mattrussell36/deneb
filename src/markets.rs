use serde::{Deserialize, Serialize};
use serde_json::from_str;
use serde_wasm_bindgen::to_value;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use wasm_bindgen::prelude::*;
use types::market::{Market, MarketsResult, TradableInstrument, Instrument};
use web_sys::HtmlInputElement;


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct ListMarketsArgs<'a> {
    name: &'a str,
}

#[derive(Properties, PartialEq, Clone)]
pub struct MarketsProps {
    pub on_market_change: Callback<String>,
}

pub struct Markets {
    markets: Vec<Market>,
    loading: bool,
    search_term: String,
    props: MarketsProps,
}

pub enum Msg {
    GetMarkets,
    RecieveMarkets(MarketsResult),
    SetSearchTerm(String),
    SelectMarket(String)
}

impl Component for Markets {
    type Message = Msg;
    type Properties = MarketsProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self { 
            markets: vec![],
            loading: false,
            search_term: String::from(""),
            props: ctx.props().clone()
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link().clone();
        match msg {
            Msg::GetMarkets => {
                self.loading = true;
                spawn_local(async move {
                    let result = get_markets().await;
                    link.send_message(Msg::RecieveMarkets(result));
                });
                true
            },
            Msg::RecieveMarkets(data) => {
                self.markets = data.markets;
                self.loading = false;
                true
            },
            Msg::SetSearchTerm(term) => {
                self.search_term = term;
                true
            },
            Msg::SelectMarket(market_id) => {
                self.props.on_market_change.emit(market_id);
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match self.loading {
            true => {
                html! {
                    <div class="p-4">
                        <h1>{"Loading"}</h1>
                    </div>
                }
            },
            false => {
                let link = ctx.link();
                let term = &self.search_term.to_uppercase();
                let markets = filter_markets(&self.markets, &term.to_string());

                html! {
                    <div class="h-full grid grid-rows-[min-content_1fr]">
                        <div>
                            <input 
                                oninput={link.callback(|e: InputEvent| {
                                    let input: HtmlInputElement = e.target_unchecked_into();
                                    let value = input.value();
                                    Msg::SetSearchTerm(value)
                                })} 
                                type="text"
                                class="px-4 py-2 w-full outline-none focus:bg-slate-100"
                                placeholder="Search..."
                                autocomplete="off"
                            /> 
                        </div>
                        <div class="overflow-auto">
                            <ul>
                            { for markets.iter().cloned().map(|m| {
                                html! {
                                    <li class="border-b border-slate-100">
                                        <button
                                            onclick={link.callback(move |_: MouseEvent| {
                                                let id = m.id.clone();
                                                Msg::SelectMarket(id)
                                            })}
                                            class="w-full p-4 text-left"
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
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        let link = ctx.link();
        if first_render {
            link.send_message(Msg::GetMarkets);
        }
    }
}


async fn get_markets() -> MarketsResult {
    let args = to_value(&{}).unwrap();
    let result = invoke("get_markets", args).await.as_string().unwrap();
    from_str(&result).unwrap()
}

fn filter_markets(markets: &Vec<Market>, term: &str) -> Vec<Market> {
    markets
        .iter()
        .cloned()
        .filter(|m| m.tradable_instrument.instrument.code.contains(&term))
        .collect::<Vec<_>>()
}

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