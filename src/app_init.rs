use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use types::market::{Market, MarketsResult};
use tauri_sys::tauri;

use crate::market_list::MarketList;
use crate::market_view::MarketView;
use crate::utils::console_log;

#[derive(Serialize)]
struct GetMarketCmdArgs {
    name: String,
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

impl Component for AppInit {
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
                    let result = tauri::invoke("get_markets", &GetMarketCmdArgs { name: "Foo".to_string() })
                        .await
                        .unwrap();
                    link.send_message(Msg::RecieveMarkets(result));
                });
                true
            },
            Msg::RecieveMarkets(data) => {
                console_log(format!("recieved {} markets", data.markets.len()));
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
                let market = self.markets
                    .iter()
                    .find(|m| m.id == self.market_id)
                    .cloned();

                html! {
                    <div class="w-screen h-screen grid grid-cols-[300px_1fr] border-t border-slate-200">
                        <div class="min-h-0 border-r border-slate-200">
                            <MarketList
                                markets={self.markets.clone()}
                                market_id={self.market_id.clone()}
                                on_market_change={link.callback(|id| {
                                    Msg::SelectMarket(String::from(id))
                                })}
                            />
                        </div>
                        <main class="min-h-0">
                            {match market {
                                Some(m) => {
                                    html! {
                                        <MarketView market={m} />
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

