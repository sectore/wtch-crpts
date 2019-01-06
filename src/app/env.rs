use dotenv::dotenv;
use std::env;
use std::sync::{Once, ONCE_INIT};

use super::errors;

static INIT_ENV: Once = ONCE_INIT;
pub const ENV_COINMARKETCAP_KEY: &str = "COINMARKETCAP_KEY";
pub const HEADER_COINMARKETCAP_KEY: &str = "X-CMC_PRO_API_KEY";

pub fn get_env(key: &str) -> Result<String, errors::AppError> {
    INIT_ENV.call_once(|| {
        dotenv().ok();
    });
    env::var(key.to_string()).map_err(|_| errors::AppError::Env { name: key.into() })
}

#[derive(Debug)]
pub struct Env<'a> {
    pub fiat_symbol: &'a str,
    pub crypto_symbols: Vec<&'a str>,
    pub is_development: bool,
}

impl<'a> Env<'a> {
    pub fn new(crypto_symbols: Vec<&'a str>, fiat_symbol: &'a str, is_development: bool) -> Self {
        Env {
            crypto_symbols,
            fiat_symbol,
            is_development,
        }
    }
}
