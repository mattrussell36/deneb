
pub mod market {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    pub enum TradingMode {
        /// Default value, this is invalid
        Unspecified = 0,
        /// Normal trading
        Continuous = 1,
        /// Auction trading (FBA)
        BatchAuction = 2,
        /// Opening auction
        OpeningAuction = 3,
        /// Auction triggered by monitoring
        MonitoringAuction = 4,
        /// No trading is allowed
        NoTrading = 5,
    }

    impl std::fmt::Display for TradingMode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                TradingMode::Unspecified => write!(f, "Unspecified"),
                TradingMode::Continuous => write!(f, "Continuous"),
                TradingMode::BatchAuction => write!(f, "Batch auction"),
                TradingMode::OpeningAuction => write!(f, "Opening auction"),
                TradingMode::MonitoringAuction => write!(f, "Monitoring auction"),
                TradingMode::NoTrading => write!(f, "No trading"),
            }
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    pub enum State {
        /// Default value, invalid
        Unspecified = 0,
        /// The governance proposal valid and accepted
        Proposed = 1,
        /// Outcome of governance votes is to reject the market
        Rejected = 2,
        /// Governance vote passes/wins
        Pending = 3,
        /// Market triggers cancellation condition or governance
        /// votes to close before market becomes Active
        Cancelled = 4,
        /// Enactment date reached and usual auction exit checks pass
        Active = 5,
        /// Price monitoring or liquidity monitoring trigger
        Suspended = 6,
        /// Governance vote to close (Not currently implemented)
        Closed = 7,
        /// Defined by the product (i.e. from a product parameter,
        /// specified in market definition, giving close date/time)
        TradingTerminated = 8,
        /// Settlement triggered and completed as defined by product
        Settled = 9,
    }

    impl std::fmt::Display for State {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                State::Unspecified => write!(f, "Unspecified"),
                State::Proposed => write!(f, "Proposed"),
                State::Rejected => write!(f, "Rejected"),
                State::Pending => write!(f, "Pending"),
                State::Cancelled => write!(f, "Cancelled"),
                State::Active => write!(f, "Active"),
                State::Suspended => write!(f, "Suspended"),
                State::Closed => write!(f, "Closed"),
                State::TradingTerminated => write!(f, "Trading terminated"),
                State::Settled => write!(f, "Settled"),
            }
        }
    }

    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
    pub struct Market {
        pub id: String,
        pub decimal_places: u64,
        pub position_decimal_places: i64,
        pub trading_mode: TradingMode,
        pub state: State,
        pub instrument_code: String,
        pub instrument_name: String,
    }

    #[derive(Debug, Clone, Deserialize, Serialize)]
    pub struct MarketsResult {
        pub markets: Vec<Market>
    }

    #[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
    pub struct EventPayload {
        pub num: u16,
        pub message: String
    }
}

