use serde_json::Value;
use tokio_tungstenite::connect_async;
use futures_util::StreamExt;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Trade {
    s: String,  // symbol
    p: String,  // price
    q: Option<String>, // quantity
}

#[derive(Debug, Deserialize)]
struct BookTicker {
    s: String,  // symbol
    b: String,  // best bid
    a: String,  // best ask
}



#[tokio::main]
async fn main() {
    let url = "wss://stream.binance.com:9443/stream?streams=btcusdt@trade/ethusdt@bookTicker";

    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");

    let (_, mut read) = ws_stream.split();

    while let Some(msg) = read.next().await {
        let msg = match msg {
            Ok(msg) => msg.into_text().unwrap(),
            Err(e) => {
                println!("WebSocket error: {}", e);
                continue;
            }
        };

        // Parse the JSON dynamically
        let parsed: Value = match serde_json::from_str(&msg) {
            Ok(p) => p,
            Err(e) => {
                println!("Deserialize error: {}, raw: {}", e, msg);
                continue;
            }
        };

        // Extract stream name and data
        let stream_name = match parsed["stream"].as_str() {
            Some(s) => s,
            None => {
                println!("Missing stream field: {}", msg);
                continue;
            }
        };
        let data = &parsed["data"];

        // Match based on stream type
        match stream_name.split('@').nth(1).unwrap_or("") {
            "trade" => {
                if let Ok(trade) = serde_json::from_value::<Trade>(data.clone()) {
                    println!("TRADE: {} price {}", trade.s, trade.p);
                }
            }
            "bookTicker" => {
                if let Ok(book) = serde_json::from_value::<BookTicker>(data.clone()) {
                    println!("BOOK: {} bid {} ask {}", book.s, book.b, book.a);
                }
            }
            _ => {
                println!("Unknown stream: {}", stream_name);
            }
        }
    }
}