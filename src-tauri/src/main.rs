// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri_plugin_log;
use log::info;
use vega_protobufs::datanode::api::v2::{
    trading_data_service_client::TradingDataServiceClient, ListMarketsRequest, ListMarketsResponse
};
use types::market::{Market, MarketsResult, Instrument, TradableInstrument};

const NODE_ADDRESS: &str = "tcp://n06.testnet.vega.xyz:3007";

#[tauri::command]
fn greet(name: &str) -> String {
    info!("og greet: {}", name);
    return format!("Hello, {}! You've been greeted from Rust!", name);
}

async fn list_markets() -> Result<ListMarketsResponse, Box<dyn std::error::Error>> {
    let mut client = TradingDataServiceClient::connect(NODE_ADDRESS).await?;

    let resp = client
        .list_markets(ListMarketsRequest {
            pagination: None,
            include_settled: Some(false),
        })
        .await?;

    let edges = &resp.get_ref().markets.as_ref().unwrap().edges;
    let count = edges.len();
    info!("found {} markets", count);


    // for mkt in resp.get_ref().markets.as_ref().unwrap().edges.iter() {
    //     info!("{:#?}", mkt.node.as_ref().unwrap());
    // }
    let x = resp.get_ref().clone();
    Ok(x)
}

#[tauri::command]
async fn get_markets() -> Result<String, String> {
    info!("listing!");
    let result = list_markets().await;
    match result {
        Ok(data) => {
            // convert markets response into struct that is serializable
            // to allow sending to the frontend
            let markets = data.markets.as_ref().unwrap().edges.iter().map(|edge| {
                let node = edge.node.as_ref().unwrap();
                let instrument = node.tradable_instrument
                    .as_ref()
                    .unwrap()
                    .instrument
                    .as_ref()
                    .unwrap();
                Market {
                    id: node.id.clone(),
                    decimal_places: node.decimal_places,
                    position_decimal_places: node.position_decimal_places,
                    tradable_instrument: TradableInstrument {
                        instrument: Instrument {
                            id: instrument.id.clone(),
                            code: instrument.code.clone(),
                            name: instrument.name.clone(),
                        }
                    } 
                }
            }).collect();
            let result = MarketsResult {
                markets: markets,
            };
            let json = serde_json::to_string(&result).unwrap();
            Ok(json)
        },
        Err(_error) => {
            Err("failed to get markets".into())
        }
    }
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
