use super::coin::*;
use serde_json;
use std::thread::sleep;
use std::time::{Duration, Instant};
use ureq::{Agent, AgentBuilder, Response};

pub struct PriceObtainer {
    last_request_time: Instant,
    agent: Agent,
}
impl PriceObtainer {
    pub fn new() -> PriceObtainer {
        PriceObtainer {
            last_request_time: Instant::now(),
            agent: ureq::AgentBuilder::new().build().unwrap(),
        }
    }

    pub fn get_price(&mut self, coin: Coin) -> f64 {
        if last_request_time.elapsed().as_secs() <= 2 {
            sleep(Duration::new(2));
        }
        *self.last_request_time = Instant::now();
        let response: serde_json::Value = agent
            .get(format!(
                "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=USD",
                coin.coin_id()
            ))
            .call()
            .unwrap()
            .into_json()
            .unwrap();
        respone[coin.coin_id()]["usd"] as f64
    }
}
