use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
// use yew::{prelude::*, html::IntoPropValue};
use log::info;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[derive(Serialize, Deserialize)]
struct LogArgs<'a> {
    message: &'a str,
}

async fn send_log(message: &str) {
    let args = to_value(&LogArgs { message: &message}).unwrap();
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    let ret_msg = invoke("greet", args).await.as_string().unwrap();
    info!("ret_msg: {}", ret_msg);
}

#[function_component(App)]
pub fn app() -> Html {
    // let greet_input_ref = use_node_ref();

    // let name = use_state(|| String::new());

    // let greet_msg = use_state(|| String::new());
    // {
    //     let greet_msg = greet_msg.clone();
    //     let name = name.clone();
    //     let name2 = name.clone();
    //     use_effect_with_deps(
    //         move |_| {
    //             spawn_local(async move {
    //                 if name.is_empty() {
    //                     return;
    //                 }

    //                 let args = to_value(&GreetArgs { name: &*name }).unwrap();
    //                 // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    //                 let new_msg = invoke("greet", args).await.as_string().unwrap();
    //                 greet_msg.set(new_msg);
    //             });

    //             || {}
    //         },
    //         name2,
    //     );
    // }

    // let greet = {
    //     let name = name.clone();
    //     let greet_input_ref = greet_input_ref.clone();
    //     Callback::from(move |e: SubmitEvent| {
    //         e.prevent_default();
    //         info!("greet");
    //         name.set(
    //             greet_input_ref
    //                 .cast::<web_sys::HtmlInputElement>()
    //                 .unwrap()
    //                 .value(),
    //         );
    //     })
    // };

    let input_value_state = use_state(|| String::from("bar"));
    let input_value = (*input_value_state).clone();
    let input_value_text = (*input_value_state).clone();
    let input_value_submission = (*input_value_state).clone();

    let oninput = {
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let value = input.value();
            input_value_state.set(value);
        })
    };

    let onsubmit = {
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            info!("submitting {}", input_value_submission);
            // let args = to_value(&LogArgs { message: "hello from fe" }).unwrap();
            // let res = invoke("mylog", args).await.as_string().unwrap();
            // send_log(&input_value_submission).await;
        })
    };

    html! {
        <main class="container bg-black">
            <form class="row" {onsubmit}> 
                <input value={input_value} {oninput} placeholder="Enter a name..." />
                <button type="submit">{"Submit"}</button>
            </form>
            <div>{"text: "}{input_value_text}</div>
        </main>
    }
}
