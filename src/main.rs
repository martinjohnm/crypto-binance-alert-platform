mod market_manager;
mod alert_manager;

use tokio_tungstenite::connect_async;
use futures_util::StreamExt;
use serde::Deserialize;

use crate::market_manager::MarketManager;

#[derive(Debug, Deserialize)]
struct Trade {
    s: String,  // symbol
    p: String,  // price
    q: String, // quantity
}

#[derive(Debug, Deserialize)]
struct BookTicker {
    s: String,  // symbol
    b: String,  // best bid
    a: String,  // best ask
}

#[derive(Deserialize, Debug)]
struct BinanceEnvelope {
    pub stream: String,
    pub data: BinanceData
}

#[derive(Deserialize, Debug)]
#[serde(untagged)] // Serde will try to match the fields to see which one it is
pub enum BinanceData {
    Trade(Trade),
    BookTicker(BookTicker),
}



#[tokio::main]
async fn main() {
    let url = "wss://stream.binance.com:9443/stream?streams=btcusdt@trade/ethusdt@trade/solusdt@trade/bnbusdt@trade/adausdt@trade/xrpusdt@trade/dotusdt@trade/dogeusdt@trade/avaxusdt@trade/shibusdt@trade/maticusdt@trade/ltcusdt@trade/nearusdt@trade/ftmusdt@trade/atomusdt@trade/linkusdt@trade/trxusdt@trade/uniusdt@trade/bchusdt@trade/axsusdt@trade";
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");

    let (_, mut read) = ws_stream.split();

    let manager = MarketManager::new();

    while let Some(msg) = read.next().await {
        let msg_text = match msg {
            Ok(m) => m.into_text().unwrap(),
            Err(e) => {
                println!("WebSocket error: {}", e);
                continue;
            }
        };
        // println!("{}", msg_text);
        let parsed = match serde_json::from_str::<BinanceEnvelope>(&msg_text) {
            Ok(p) => p,
            Err(e) => {
                println!("Deserialize error: {}", e);
                continue;
            }
        };

        match parsed.data {
            BinanceData::Trade(t) => {
                // No need to manually check "stream" name if your Enum variants 
                // are already correctly deserialized!
                let price: f64 = t.p.parse().unwrap_or(0.0);
                
                manager.update_price(&t.s, price);
                // println!("🚀 {} updated to {}", t.s, price);
            }
            BinanceData::BookTicker(b) => {
                // println!("📖 {} Bid: {} | Ask: {}", b.s, b.b, b.a);
            }
        }

        println!("{:?}", manager)
        // // Parse the JSON dynamically
        // let parsed: Value = match serde_json::from_str(&msg_text) {
        //     Ok(p) => p,
        //     Err(e) => {
        //         println!("Deserialize error: {}, raw: {}", e, &msg_text);
        //         continue;
        //     }
        // };

        // // Extract stream name and data
        // let stream_name = match parsed["stream"].as_str() {
        //     Some(s) => s,
        //     None => {
        //         println!("Missing stream field: {}", &msg_text);
        //         continue;
        //     }
        // };
        // let data = match parsed["data"].as_str() {
        //     Some(s) => s,
        //     None => {
        //         println!("Missing stream field: {}", &msg_text);
        //         continue;
        //     }
        // };

        // // Match based on stream type
        // match stream_name.split('@').nth(1).unwrap_or("") {
        //     "trade" => {
        //         if let Ok(trade) = serde_json::from_str::<Trade>(&parsed) {
        //             let price : f64 = trade.p.parse().unwrap();
        //             manager.update_price(&trade.s, price);
        //             println!("{:?}",manager );
        //         }
        //     }
        //     "bookTicker" => {
        //         if let Ok(book) = serde_json::from_str::<BookTicker>(&parsed) {
        //             println!("BOOK: {} bid {} ask {}", book.s, book.b, book.a);
        //         }
        //     }
        //     _ => {
        //         println!("Unknown stream: {}", stream_name);
        //     }
        // }
    }
}