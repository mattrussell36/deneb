// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri_plugin_log;
use log::info;
use vega_protobufs::datanode::api::v2::{
    trading_data_service_client::TradingDataServiceClient, ListMarketsRequest, ListMarketsResponse
};

const NODE_ADDRESS: &str = "tcp://n06.testnet.vega.xyz:3007";

#[tauri::command]
fn greet(name: &str) -> String {
    info!("og greet: {}", name);
    return format!("Hello, {}! You've been greeted from Rust!", name);
}

async fn list_markets() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = TradingDataServiceClient::connect(NODE_ADDRESS).await?;

    let resp = client
        .list_markets(ListMarketsRequest {
            pagination: None,
            include_settled: Some(false),
        })
        .await?;

    for mkt in resp.get_ref().markets.as_ref().unwrap().edges.iter() {
        info!("{:?}", mkt.node.as_ref().unwrap());
    }

    Ok(())
}

#[tauri::command]
async fn get_markets() {
    info!("listing!");
    list_markets().await;
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            greet,
            get_markets,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
