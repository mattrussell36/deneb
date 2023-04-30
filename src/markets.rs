use serde::{Deserialize, Serialize};
use serde_json::from_str;
use serde_wasm_bindgen::to_value;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use wasm_bindgen::prelude::*;
use types::market::{Market, MarketsResult};


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct ListMarketsArgs<'a> {
    name: &'a str,
}

pub struct Markets {
    markets: Vec<Market>,
    loading: bool,
}

pub enum Msg {
    GetMarkets,
    RecieveMarkets(MarketsResult)
}

impl Component for Markets {
    type Message = Msg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self { 
            markets: vec![],
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
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        match self.loading {
            true => {
                html! {
                    <h1>{"Loading"}</h1>
                }
            },
            false => {
                html! {
                    <div class="h-full overflow-auto">
                        <ul>
                        { for self.markets.iter().map(|m| html! {
                            <li class="p-4 border-b border-slate-100">
                                <div>{ &m.tradable_instrument.instrument.code }</div>
                            </li>
                        }) }
                        </ul> 
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