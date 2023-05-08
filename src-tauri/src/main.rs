// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod store;
mod helpers;
mod markets;

extern crate directories;

use std::sync::Mutex;

use tauri_plugin_log;
use log::info;
use tauri::{Window, async_runtime::JoinHandle};
use types::market::{
    Market,
    MarketsResult,
};

pub struct AppStateInner {
    pub handle: Option<JoinHandle<()>>
} 

impl AppStateInner {
    pub fn connect(&mut self, window: Window, id: String) {
        self.abort_handle();

        let handle = tauri::async_runtime::spawn(async move {
            let _ = markets::subscribe(&id, window)
                .await;
        });

        self.handle = Some(handle);         
    }
    pub fn disconnect(&self) {
        self.abort_handle(); 
    }
    pub fn abort_handle(&self) {
        match &self.handle {
            Some(handle) => {
                handle.abort()
            },
            None => {
                // no op
            }
        } 
    }
}

pub struct AppState(pub Mutex<AppStateInner>);



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
fn subscribe_to_market(
    window: Window,
    id: String,
    state: tauri::State<AppState>
) -> Result<String, String> {
    info!("subscribing: {}", id);
    let mut state_guard = state.0.lock().unwrap();

    state_guard.connect(window, id);

    Ok("added".to_string())
}

#[tauri::command]
fn unsubscribe_to_market(
    _window: Window,
    id: String,
    state: tauri::State<AppState>
) -> Result<String, String> {
    info!("unsubscribing: {}", id);
    let state_guard = state.0.lock().unwrap();

    state_guard.disconnect();

    Ok("removed".to_string())
}

fn main() {
    tauri::Builder::default()
        .manage(AppState(Mutex::new(AppStateInner { handle: None } )))
        .invoke_handler(tauri::generate_handler![
            greet,
            get_markets,
            subscribe_to_market,
            unsubscribe_to_market,
        ])
        .plugin(tauri_plugin_log::Builder::default().build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}