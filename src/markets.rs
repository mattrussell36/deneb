use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use log::info;
use wasm_bindgen::prelude::*;

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
    message: String,
}

pub enum Msg {
  GetMarkets
}

impl Component for Markets {
    type Message = Msg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self { 
            message: String::from("bar"),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
      match msg {
        Msg::GetMarkets => {
          spawn_local(async move {
            let args = to_value(&ListMarketsArgs { name: "asdf" }).unwrap();
            invoke("get_markets", args).await;
          });
          true
        }
      }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
      let link = ctx.link();
        html! {
            <div class="container bg-black">
              <h1>{"Markets"}</h1>
              <h2>{"Message "}{&self.message}</h2>
              <button onclick={link.callback(|e: MouseEvent| {
                Msg::GetMarkets
              })}>{"List markets"}</button>
            </div>
        }
    }
}
