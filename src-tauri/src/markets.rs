use log::info;
use tonic::transport::Channel;
use vega_protobufs::datanode::api::v2::{
    trading_data_service_client::TradingDataServiceClient,
    ListMarketsRequest,
    ListMarketsResponse,
    ObserveMarketsDataRequest,
};
use types::market::{
    MarketData
};
use futures::stream::StreamExt;
use tauri::Window;
use std::io::{Error, ErrorKind};

const NODE_ADDRESS: &str = "tcp://n06.testnet.vega.xyz:3007";

pub async fn list_markets() -> Result<ListMarketsResponse, Box<dyn std::error::Error>> {
    let mut client: TradingDataServiceClient<Channel> = create_client().await;

    let resp = client
        .list_markets(ListMarketsRequest {
            pagination: None,
            include_settled: Some(false),
        })
        .await?;

    let edges = &resp.get_ref().markets.as_ref().unwrap().edges;
    let count = edges.len();

    info!("found {} markets", count);

    let x = resp.get_ref().clone();
    Ok(x)
}

pub async fn subscribe(id: &str, window: Window) -> Result<(), Error> {
    info!("subscribing inside {}", id);

    let mut client: TradingDataServiceClient<Channel> = create_client().await;

    let resp = client.observe_markets_data(ObserveMarketsDataRequest {
        market_ids: vec![id.to_string()]
    })
        .await
        .expect("failed to observe market data");

    let mut stream = resp.into_inner();

    while let Some(data) = stream.next().await {
        let markets = data.unwrap().market_data;

        if markets.len() > 1 {
            return Err(Error::new(ErrorKind::Other, "more than one market"));
        }

        let market = &markets[0];

        let d = MarketData {
            mark_price: market.mark_price.clone(),
            best_bid_price: market.best_bid_price.clone(),
            best_bid_volume: market.best_bid_volume.clone(),
            best_offer_price: market.best_offer_price.clone(),
            best_offer_volume: market.best_offer_volume.clone(),
            mid_price: market.mid_price.clone()
        };
        window.emit("market-data", d);
    }

    Ok(())
}

pub async fn unsubscribe() {
    info!("unsub");
}

async fn create_client() -> TradingDataServiceClient<Channel> {
    TradingDataServiceClient::connect(NODE_ADDRESS)
        .await
        .expect("could not create client")
}