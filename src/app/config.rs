use super::api::{Api};

type ApiBox = Box<dyn Api>;

pub struct Config<'a> {
    pub fiat_symbol: &'a str,
    pub crypto_symbols: Vec<&'a str>,
    pub is_development: bool,
    pub api: ApiBox
}

impl<'a> Config<'a> {
    pub fn new(crypto_symbols: Vec<&'a str>, fiat_symbol: &'a str, is_development: bool, api: ApiBox) -> Self {
        Config {
            crypto_symbols,
            fiat_symbol,
            is_development,
            api
        }
    }
}
