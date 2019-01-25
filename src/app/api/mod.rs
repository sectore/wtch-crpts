use super::types::{AppResult, CoinDetail, CoinList};

pub mod coinmarketcap;

pub trait Api {
        fn get_coins(&self) -> AppResult<CoinList>;
        fn get_coin_detail(&self, symbol: &String, fiat: &str) -> AppResult<CoinDetail>;
        fn get_endpoint(&self) -> &str;
}