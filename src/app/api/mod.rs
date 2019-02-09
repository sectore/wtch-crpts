use serde::de::DeserializeOwned;
use super::types::{AppResult, Coin, Coins};

pub mod coinmarketcap;

pub trait Api {
        type ApiCoin: DeserializeOwned;
        // fn get_coins(&self) -> AppResult<CoinList>;
        fn get_coin_details(&self, symbols: &[&str], fiat: &str) -> AppResult<Coins>;
        fn get_endpoint(&self) -> &str;
        fn to_coin(&self, api_coin: &Self::ApiCoin, fiat: &str) -> Coin;
}