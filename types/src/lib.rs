
pub mod market {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
    pub struct Market {
        pub id: String,
        pub decimal_places: u64,
        pub position_decimal_places: i64,
        pub tradable_instrument: TradableInstrument 
    }

    #[derive(Debug, Clone, Deserialize, Serialize)]
    pub struct MarketsResult {
        pub markets: Vec<Market>
    }


    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
    pub struct TradableInstrument {
        pub instrument: Instrument,
    }

    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
    pub struct Instrument {
        pub id: String,
        pub code: String,
        pub name: String,
        // pub metadata: InstrumentMetadata,
        // pub product: Product,
    }
    
    #[derive(Debug, Clone, Deserialize, Serialize)]
    pub struct InstrumentMetadata {
        pub tags: Vec<String>,
    }

    #[derive(Debug, Clone, Deserialize, Serialize)]
    pub enum Product {
        Future(Future)
    }

    #[derive(Debug, Clone, Deserialize, Serialize)]
    pub struct Future {
        pub settlement_asset: String,
        pub quote_name: String,
    }
}