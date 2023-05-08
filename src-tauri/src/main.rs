// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod store;
mod helpers;
mod markets;

extern crate directories;

use tauri_plugin_log;
use log::info;
use tauri::Window;
use types::market::{
    Market,
    MarketsResult,
    EventPayload
};

#[tauri::command]
fn greet(name: &str) -> String {
    info!("og greet: {}", name);
    return format!("Hello, {}! You've been greeted from Rust!", name);
}

#[tauri::command]
async fn get_markets() -> Result<MarketsResult, String> {
    info!("get_markets");
    let stored_markets = store::get_markets();

    match stored_markets {
        Ok(data) => {
            if data.len() > 0 {
                let result = MarketsResult {
                    markets: data,
                };
                return Ok(result);
            }
        },
        Err(_) => {
            info!("no stored markets");
        }
    }

    info!("fetching markets");
    let result = markets::list_markets().await;

    match result {
        Ok(data) => {
            // convert markets response into struct that is serializable
            // to allow sending to the frontend
            let markets: Vec<Market> = data.markets
                .as_ref()
                .unwrap()
                .edges
                .iter()
                .map(helpers::parse_market)
                .collect();

            store::set_markets(markets.clone()).unwrap();

            let result = MarketsResult {
                markets: markets,
            };
            Ok(result)
        },
        Err(_error) => {
            Err("failed to get markets".into())
        }
    }
}

#[tauri::command]
fn emit_event(window: Window, num: u16) {
    info!("emit_event called {}", num);
    window
        .emit("my-event", EventPayload { message: String::from("foo"), num: 5 })
        .unwrap();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            get_markets,
            emit_event
        ])
        .plugin(tauri_plugin_log::Builder::default().build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}