use serde::{Deserialize, Serialize};
use serde_json::from_str;
use serde_wasm_bindgen::to_value;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use wasm_bindgen::prelude::*;
use types::market::{Market, MarketsResult};
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

pub struct Markets {
    markets: Vec<Market>,
    loading: bool,
    search_term: String,
}

pub enum Msg {
    GetMarkets,
    RecieveMarkets(MarketsResult),
    SetSearchTerm(String)
}

impl Component for Markets {
    type Message = Msg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self { 
            markets: vec![],
            loading: false,
            search_term: String::from("")
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
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        match self.loading {
            true => {
                html! {
                    <h1>{"Loading"}</h1>
                }
            },
            false => {
                let term = &self.search_term.to_uppercase();
                let markets = self.markets
                    .iter()
                    .filter(|m| m.tradable_instrument.instrument.code.contains(term))
                    .collect::<Vec<_>>();

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
                            { for markets.iter().map(|m| html! {
                                <li class="p-4 border-b border-slate-100">
                                    <div>{ &m.tradable_instrument.instrument.code }</div>
                                </li>
                            }) }
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