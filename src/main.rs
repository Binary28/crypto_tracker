use handler::Handler;
use serde_json::Value;

mod alertor;
mod error;
mod handler;

use crate::handler::Coin;

#[tokio::main]
async fn main() {
    let doge = Coin::new("dogeinr", "Doge Coin", 6.0, 1.0);
    let uni = Coin::new("uniinr", "UniSwap Coin", 1700.0, 1.0);
    let ada = Coin::new("adainr", "Cardadano(ADA) Coin", 72.0, 1.0);
    let alert = match alertor::Alerter::new("tharunkumar77tk@gmail.com") {
        Ok(val) => val,
        Err(e) => return println!("{}", e),
    };
    let mut handler = Handler::new(vec![doge, uni, ada], alert);
    loop {
        let resp = reqwest::Client::new()
            .get("https://api.wazirx.com/api/v2/tickers")
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        let data: Value = serde_json::from_str(&resp).unwrap();

        match handler.run(data).await {
            Ok(_) => continue,
            Err(e) => println!("{}", e),
        };
    }
}
