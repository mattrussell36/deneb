use log::info;
use vega_protobufs::datanode::api::v2::{
    trading_data_service_client::TradingDataServiceClient,
    ListMarketsRequest,
    ListMarketsResponse,
};

const NODE_ADDRESS: &str = "tcp://n06.testnet.vega.xyz:3007";

pub async fn list_markets() -> Result<ListMarketsResponse, Box<dyn std::error::Error>> {
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

    let x = resp.get_ref().clone();
    Ok(x)
}