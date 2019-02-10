use super::types::{AppResult, Coins};

pub mod coinmarketcap;
pub mod cryptocompare;

pub trait Api {
    fn get_coin_details(&self, symbols: &[&str], fiat: &str) -> AppResult<Coins>;
    fn get_endpoint(&self) -> &str;
}
