use vega_protobufs::datanode::api::v2::{
    MarketEdge
};
use types::market::{
    Market,
    TradingMode,
    State,
};

// takes a raw `MarketEdge` from generated structs and returns
// a flattened `Market`
pub fn parse_market(edge: &MarketEdge) -> Market {
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
        instrument_code: instrument.code.clone(),
        instrument_name: instrument.name.clone(),
        // convert i32 TradingMode enum into a custom enum which has implemented
        // a Display trait, alowing trading mode to be rendered to screen
        trading_mode: match node.trading_mode {
            0 => TradingMode::Unspecified,
            1 => TradingMode::Continuous,
            2 => TradingMode::BatchAuction,
            3 => TradingMode::OpeningAuction,
            4 => TradingMode::MonitoringAuction,
            5 => TradingMode::NoTrading,
            _ => TradingMode::Unspecified,
        },
        // convert i32 MarketState enum to a custom enum which as implemented
        // a Display trait, allowing states to be rendered to screen
        state: match node.state {
            0 => State::Unspecified,
            1 => State::Proposed,
            2 => State::Rejected,
            3 => State::Pending,
            4 => State::Cancelled,
            5 => State::Active,
            6 => State::Suspended,
            7 => State::Closed,
            8 => State::TradingTerminated,
            9 => State::Settled,
            _ => State::Unspecified,
        }
    }
}