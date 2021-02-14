use alertor::Alerter;
use chrono::{DateTime, Local};
use serde_json::Value;
use std::time::Duration;

use crate::{alertor, error::Errors};

pub struct Handler<'a> {
    pub coins: Vec<Coin<'a>>,
    alertor: alertor::Alerter<'a>,
    start_time: DateTime<Local>,
}

impl<'a> Handler<'a> {
    pub fn new(coins: Vec<Coin<'a>>, alertor: Alerter<'a>) -> Handler<'a> {
        Handler {
            coins,
            alertor,
            start_time: Local::now(),
        }
    }

    pub async fn run(&mut self, data: Value) -> Result<(), Errors<'a>> {
        for coin in self.coins.iter_mut() {
            for exchange_details in coin.exchange_details.iter_mut() {
                let price = match data[exchange_details.reference]["last"].as_str() {
                    Some(val) => val.parse::<f64>().unwrap_or(0.0),
                    None => 0.0,
                };
                exchange_details.update_price(price);
                if exchange_details.validate() {
                    self.alertor.alert_mail(coin.name).await?;
                    self.alertor.alert_voice(coin.name).await;
                };
            }
        }
        self.print_price().await;
        let diff = Local::now().signed_duration_since(self.start_time);
        println!(
            "{:<48}Uptime [{:02}h::{:02}m::{:02}s]",
            "",
            if diff.num_hours() >= 24 {
                diff.num_hours() % 24
            } else {
                diff.num_hours()
            },
            if diff.num_minutes() >= 60 {
                diff.num_minutes() % 60
            } else {
                diff.num_minutes()
            },
            if diff.num_seconds() >= 60 {
                diff.num_seconds() % 60
            } else {
                diff.num_seconds()
            },
        );
        println!("{:-<70}", "");
        std::thread::sleep(Duration::from_secs(5));
        return Ok(());
    }

    pub async fn print_price(&self) {
        for coin in self.coins.iter() {
            let now = chrono::Local::now();
            let mut stdout = vec![format!("[{}]{:^25}", now.format("%H:%M:%S"), coin.name)];
            for exchange in coin.exchange_details.iter() {
                if exchange.exchange_currency == "INR" {
                    stdout.push(format!("₹ {:<20}", exchange.current_price.unwrap_or(0.0)));
                } else {
                    stdout.push(format!("$ {:<20}", exchange.current_price.unwrap_or(0.0)));
                }
            }
            println!("{}", stdout.concat());
        }
    }
}

#[allow(dead_code)]
pub struct Coin<'a> {
    exchange_details: Vec<ExchangeDetails<'a>>,
    name: &'a str,
}

impl<'a> Coin<'a> {
    pub fn new(name: &'a str, exchange_details: Vec<ExchangeDetails<'a>>) -> Coin<'a> {
        Coin {
            name,
            exchange_details,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct ExchangeDetails<'a> {
    reference: &'a str,
    exchange_currency: &'a str,
    min_value: f64,
    max_value: f64,
    current_price: Option<f64>,
}

impl<'a> ExchangeDetails<'a> {
    pub fn new(
        reference: &'a str,
        exchange_currency: &'a str,
        min_value: f64,
        max_value: f64,
    ) -> ExchangeDetails<'a> {
        ExchangeDetails {
            reference,
            exchange_currency,
            min_value,
            max_value,
            current_price: None,
        }
    }

    pub fn update_price(&mut self, price: f64) {
        self.current_price = Some(price);
    }

    fn validate(&self) -> bool {
        if self.current_price.unwrap() >= self.max_value {
            true
        } else {
            false
        }
    }
}
