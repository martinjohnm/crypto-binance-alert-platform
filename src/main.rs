use tokio_tungstenite::connect_async;
use futures_util::StreamExt;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
struct Trade {
    s: String,   // s
    p: String,    // p
    q: Option<String>, // q
}

impl Default for Trade {
    fn default() -> Self {
        Trade {
            s: "".to_string(),
            p: "".to_string(),
            q: None,
        }
    }
}


#[tokio::main]
async fn main() {
    let url = String::from("wss://stream.binance.com:9443/ws/btcusdt@trade");

    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");

    let (_, mut read) = ws_stream.split();

    while let Some(msg) = read.next().await {
        let msg = msg.unwrap().into_text().unwrap();
        // Option 1: Deserialize into struct
        let trade: Result<Trade, _> = serde_json::from_str(&msg);

        match trade {
            Ok(t) => println!("{} price {}", t.s, t.p),
            Err(e) => {
                // Option 2: Fallback: just print raw JSON
                println!("Deserialize error: {}, raw: {}", e, msg);
            }
        }
    }
}