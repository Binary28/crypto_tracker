use std::time::Duration;

use alertor::Alerter;
use serde_json::Value;

use crate::{alertor, error::Errors};

pub struct Handler<'a> {
    pub coins: Vec<Coin<'a>>,
    alertor: alertor::Alerter<'a>,
}

impl<'a> Handler<'a> {
    pub fn new(coins: Vec<Coin<'a>>, alertor: Alerter<'a>) -> Handler<'a> {
        Handler { coins, alertor }
    }

    pub async fn run(&mut self, data: Value) -> Result<(), Errors<'a>> {
        for coin in self.coins.iter() {
            let price = match data[coin.reference]["last"].as_str() {
                Some(val) => val.parse::<f64>().unwrap_or(0.0),
                None => 0.0,
            };
            let now = chrono::Local::now();
            println!(
                " [{:20}] Price: {:^10}{:>5}[{}]",
                coin.name,
                price,
                "",
                now.format("%b %-d %-I:%M").to_string()
            );
            if coin.validate(price) {
                self.alertor.alert_mail(coin.name).await?;
                self.alertor.alert_voice(coin.name).await;
            }
        }
        println!("{:-<60}", "");
        std::thread::sleep(Duration::from_secs(5));
        return Ok(());
    }
}

#[allow(dead_code)]
pub struct Coin<'a> {
    reference: &'a str,
    name: &'a str,
    max_value: f64,
    min_value: f64,
}

impl<'a> Coin<'a> {
    pub fn new(reference: &'a str, name: &'a str, max_value: f64, min_value: f64) -> Coin<'a> {
        Coin {
            reference,
            name,
            max_value,
            min_value,
        }
    }

    fn validate(&self, price: f64) -> bool {
        if price >= self.max_value {
            true
        } else {
            false
        }
    }
}
