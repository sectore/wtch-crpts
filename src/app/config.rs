use super::api::Api;

#[derive(Debug)]
pub struct Config<'a, T> {
    pub fiat_symbol: &'a str,
    pub crypto_symbols: Vec<&'a str>,
    pub is_development: bool,
    pub api: T
}

impl<'a, T: Api> Config<'a, T> {
    pub fn new(crypto_symbols: Vec<&'a str>, fiat_symbol: &'a str, is_development: bool, api: T) -> Self {
        Config {
            crypto_symbols,
            fiat_symbol,
            is_development,
            api
        }
    }
}
