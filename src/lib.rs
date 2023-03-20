mod utils;
mod types;

use wasm_bindgen::prelude::*;
use rust_decimal_macros::dec;
use rust_decimal::prelude::*;
use types::*;


// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, positon-core-sdk!");
}

#[wasm_bindgen]
pub fn computeOutput(mainSize: &str, isBuy: bool, orderBookRaw: OrderBookRaw) -> &'static str {
    let flipAmount: Decimal = dec!(0.0);
    let cloneMainSize = Decimal::from_str(mainSize).unwrap();

    let orders = if isBuy { orderBookRaw.asks } else { orderBookRaw.bids };
    let price = orderBookRaw.currentPrice;

    let mut vec: Vec<str> = Vec::new();
    while cloneMainSize.gt(dec!(0)) {
        if orders.len() > 0{
            nexOrder = orders[0].clone();
        }
    }


    return "";
}
