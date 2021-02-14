use handler::{ExchangeDetails, Handler};
use serde_json::Value;

mod alertor;
mod error;
mod handler;

use crate::handler::Coin;

#[tokio::main]
async fn main() {
    let doge = Coin::new(
        "Doge Coin",
        vec![
            ExchangeDetails::new("dogeinr", "INR", 1.0, 5.2),
            ExchangeDetails::new("dogeusdt", "USDT", 1.0, 0.07),
        ],
    );
    let uni = Coin::new(
        "UniSwap Coin",
        vec![
            ExchangeDetails::new("uniinr", "INR", 1.0, 1660.0),
            ExchangeDetails::new("uniusdt", "USDT", 1.0, 22.5),
        ],
    );
    let ada = Coin::new(
        "Cardadano(ADA) Coin",
        vec![
            ExchangeDetails::new("adainr", "INR", 1.0, 68.0),
            ExchangeDetails::new("adausdt", "USDT", 0.1, 0.96),
        ],
    );

    let wst = Coin::new(
        "Waltonchain Coin",
        vec![ExchangeDetails::new("wtcusdt", "USDT", 0.1, 1.65)],
    );

    let alert = match alertor::Alerter::new("tharunkumar77tk@gmail.com") {
        Ok(val) => val,
        Err(e) => return println!("{}", e),
    };
    let mut handler = Handler::new(vec![doge, uni, ada, wst], alert);
    loop {
        let resp = match reqwest::Client::new()
            .get("https://api.wazirx.com/api/v2/tickers")
            .send()
            .await
        {
            Ok(val) => val,
            Err(e) => {
                println!("[-] CANNOT PROCESS RESPONSE {}", e.to_string());
                continue;
            }
        };
        let text = match resp.text().await {
            Ok(val) => val,
            Err(e) => {
                println!("[-] CANNOT PROCESS RESPONSE {}", e.to_string());
                continue;
            }
        };
        let data: Value = serde_json::from_str(&text).unwrap();

        match handler.run(data).await {
            Ok(_) => continue,
            Err(e) => println!("{}", e),
        };
    }
}
