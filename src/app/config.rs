#[derive(Debug)]
pub struct Config<'a> {
    pub fiat_symbol: &'a str,
    pub crypto_symbols: Vec<&'a str>,
    pub is_development: bool,
}

impl<'a> Config<'a> {
    pub fn new(crypto_symbols: Vec<&'a str>, fiat_symbol: &'a str, is_development: bool) -> Self {
        Config {
            crypto_symbols,
            fiat_symbol,
            is_development,
        }
    }
}
