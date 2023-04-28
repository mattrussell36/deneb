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
struct GreetArgs<'a> {
    name: &'a str,
}

pub struct App {
    message: String,
    greeting: String,
}

pub enum Msg {
    Update(String),
    Submit,
    RecieveGreeting(String)
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self { 
            message: String::from("bar"),
            greeting: String::from("default greeting"),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Update(val) => {
                info!("update");
                self.message = val;
                true
            },
            Msg::Submit => {
                let m = self.message.clone();
                let link = ctx.link().clone();
                spawn_local(async move {
                    let result = send_greet_name(&m).await;
                    link.send_message(Msg::RecieveGreeting(result));
                });
                true
            },
            Msg::RecieveGreeting(val) => {
                self.greeting = val;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <main class="container bg-black">
                <form
                    class="row"
                    onsubmit={link.callback(|e: SubmitEvent| {
                        e.prevent_default();
                        Msg::Submit
                    })}
                > 
                    <input
                        oninput={link.callback(|e: InputEvent| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            let value = input.value();
                            Msg::Update(value)
                        })}
                        placeholder="Enter a name..."
                    />
                    <button type="submit">{"Submit"}</button>
                </form>
                <div>{"text: "}{&self.message}</div>
                <div>{"greeting: "}{&self.greeting}</div>
            </main>
        }
    }
}

async fn send_greet_name(m: &str) -> String {
    let args = to_value(&GreetArgs { name: m }).unwrap();
    invoke("greet", args).await.as_string().unwrap()
}