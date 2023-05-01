use serde::{Deserialize, Serialize};
use serde_json::from_str;
use serde_wasm_bindgen::to_value;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use wasm_bindgen::prelude::*;
use types::market::{Market, MarketsResult};

use crate::market_list::MarketList;
use crate::trade::Trade;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct ListMarketsArgs<'a> {
    name: &'a str,
}

pub struct AppInit {
    markets: Vec<Market>,
    market_id: String,
    loading: bool,
}

pub enum Msg {
    GetMarkets,
    RecieveMarkets(MarketsResult),
    SelectMarket(String)
}

impl Component for AppInit{
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { 
            markets: vec![],
            market_id: String::from(""),
            loading: false,
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
            Msg::SelectMarket(market_id) => {
                self.market_id = market_id;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match self.loading {
            true => {
                html! {
                    <div class="w-screen h-screen flex justify-center items-center">
                        <h1>{"Loading"}</h1>
                    </div>
                }
            },
            false => {
                let link = ctx.link();
                let markets = self.markets.clone();
                let markets2 = self.markets.clone();
                let market = markets.iter().find(|&m| m.id == self.market_id);

                html! {
                    <div class="w-screen h-screen grid grid-cols-[300px_1fr] border-t border-slate-200">
                        <div class="min-h-0 border-r border-slate-200">
                            <MarketList markets={markets2} on_market_change={link.callback(|id| {
                                Msg::SelectMarket(String::from(id))
                            })} />
                        </div>
                        <main class="min-h-0">
                            {match market {
                                Some(m) => {
                                    let market = m.clone();
                                    html! {
                                        <Trade market={market} />
                                    }
                                },
                                None => html! {
                                    <h1 class="text-xl text-center mt-20">{"No market selected"}</h1>
                                }
                            }}
                        </main>
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