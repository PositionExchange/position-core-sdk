// struct OrderBook {
//     bids: vec<Vec<str>>,
//     asks: Vec<Vec<str>>,
//     liquidity: Vec<Vec<>>
// }

pub struct OrderBookRaw {
    pub bids: Vec<Vec<str>>,
    pub asks: Vec<Vec<str>>,
    pub liquidity: Vec<Vec<str>>,
    pub currentIndexPipRange: u64,
    pub pipSpace: u64,
    pub pipRange: u64,
    pub basisPoint: u64,
    pub currentPrice: f64,
}
