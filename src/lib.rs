extern crate reqwest;
use reqwest::Url;

#[macro_use]
extern crate log;
extern crate env_logger;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

use std::collections::HashMap;

// FIAT list supported by coinmarketcap
// https://coinmarketcap.com/api/documentation/v1/#section/Standards-and-Conventions
pub const FIAT_LIST: [&str; 93] = [
    "USD", "ALL", "DZD", "ARS", "AMD", "AUD", "AZN", "BHD", "BDT", "BYN", "BMD", "BOB", "BAM", "BRL", "BGN", "KHR",
    "CAD", "CLP", "CNY", "COP", "CRC", "HRK", "CUP", "CZK", "DKK", "DOP", "EGP", "EUR", "GEL", "GHS", "GTQ", "HNL",
    "HKD", "HUF", "ISK", "INR", "IDR", "IRR", "IQD", "ILS", "JMD", "JPY", "JOD", "KZT", "KES", "KWD", "KGS", "LBP",
    "MKD", "MYR", "MUR", "MXN", "MDL", "MNT", "MAD", "MMK", "NAD", "NPR", "TWD", "NDZ", "NIO", "NGN", "NOK", "OMR",
    "PKR", "PAB", "PEN", "PHP", "PLN", "GBP", "QAR", "RON", "RUB", "SAR", "RSD", "SGD", "ZAR", "KRW", "SSP", "VES",
    "LKR", "SEK", "CHF", "THB", "TTD", "TND", "TRY", "UGX", "UAH", "AED", "UYU", "UZS", "VND",
];

#[derive(Debug)]
pub struct Env<'a> {
    coinmarketcap_key: String,
    fiat_symbol: &'a str,
    crypto_symbols: Vec<&'a str>,
    is_development: bool,
}

impl<'a> Env<'a> {
    pub fn new(
        coinmarketcap_key: String,
        crypto_symbols: Vec<&'a str>,
        fiat_symbol: &'a str,
        is_development: bool,
    ) -> Self {
        Env {
            coinmarketcap_key,
            crypto_symbols,
            fiat_symbol,
            is_development,
        }
    }
}

#[derive(Debug)]
struct Coins {
    index: usize,
    list: Vec<Coin>,
}

impl Coins {
    fn new(list: Vec<Coin>) -> Self {
        Coins { list, index: 0 }
    }

    fn current(&self) -> Option<Coin> {
        self.list.get(self.index).map(|x| x.clone())
    }

    #[allow(dead_code)]
    fn prev(&mut self) -> Option<Coin> {
        self.index = if self.index >= 1 {
            self.index - 1
        } else {
            self.list.len() - 1
        };
        self.current()
    }
}

impl Iterator for Coins {
    type Item = Coin;
    fn next(&mut self) -> Option<Coin> {
        self.index = if self.index < self.list.len() - 1 {
            self.index + 1
        } else {
            0
        };
        self.current()
    }
}

#[derive(Debug)]
pub struct WatchCryptos<'a> {
    env: Env<'a>,
    coins: Option<Coins>,
    coin_detail: Option<CoinDetail>,
}

impl<'a> WatchCryptos<'a> {
    pub fn new(env: Env<'a>) -> Self {
        WatchCryptos {
            env,
            coins: None,
            coin_detail: None,
        }
    }

    pub fn run(&mut self) {
        self.get_coins();
        self.get_current_coin_detail();

        info!("{:?}", self);
    }

    fn get_coins(&mut self) {
        let coins = fetch_coins(&self.env.coinmarketcap_key).expect("Could not fetch list of cryptocurrencies");
        let selected_coins: Vec<Coin> = coins
            .into_iter()
            .filter(|coin| self.env.crypto_symbols.contains(&coin.symbol.as_str()))
            .collect();

        if selected_coins.is_empty() {
            panic!(format!("Cryptocurrencies {:?} are not supported", selected_coins))
        } else {
            self.coins = Some(Coins::new(selected_coins));
        }
    }

    fn get_current_coin_detail(&mut self) {
        if let Some(coins) = &self.coins {
            if let Some(item) = &coins.current() {
                let result = fetch_detail(
                    &self.env.coinmarketcap_key,
                    &item.symbol,
                    &self.env.fiat_symbol.to_string(),
                );
                if let Ok(detail) = &result {
                    self.coin_detail = Some(detail.clone());
                }
                info!("detail {:?}", &result);
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Coin {
    id: i32,
    name: String,
    symbol: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListData {
    #[serde(rename = "data")]
    coins: Vec<Coin>,
}

pub type CoinDetailMap = HashMap<String, CoinDetail>;

#[derive(Serialize, Debug, Deserialize, PartialEq)]
pub struct QuoteData {
    #[serde(rename = "data")]
    details: CoinDetailMap,
}

pub type QuoteMap = HashMap<String, Quote>;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CoinDetail {
    id: i32,
    name: String,
    symbol: String,
    #[serde(rename = "quote")]
    quotes: QuoteMap,
}

#[derive(Serialize, Debug, Deserialize, Clone, PartialEq)]
pub struct Quote {
    price: f32,
    volume_24h: f32,
}
// const URL_MAP: &str = "https://pro-api.coinmarketcap.com/v1/cryptocurrency/map";
const URL_MAP: &str = "http://localhost:3000/map";

pub type ApiError = Box<std::error::Error>;
pub type ApiResult<T> = Result<T, ApiError>;

pub fn fetch_coins(key: &String) -> ApiResult<Vec<Coin>> {
    info!("fetch coins");

    let client = reqwest::Client::new();
    let url = Url::parse_with_params(URL_MAP, &[("start", "1"), ("limit", "5000")])?;
    let data: ListData = client
        .get(url)
        .header("X-CMC_PRO_API_KEY", key.clone())
        .send()?
        .json()?;

    Ok(data.coins)
}

// const URL_QUOTES: &str = "https://pro-api.coinmarketcap.com/v1/cryptocurrency/quotes/latest";
const URL_QUOTES: &str = "http://localhost:3000/quotes";

pub fn fetch_detail(key: &String, symbol: &String, fiat: &String) -> ApiResult<CoinDetail> {
    info!("fetch detail");

    let client = reqwest::Client::new();
    let url = Url::parse_with_params(URL_QUOTES, &[("symbol", &symbol), ("convert", &fiat)])?;
    let data: QuoteData = client
        .get(url)
        .header("X-CMC_PRO_API_KEY", key.clone())
        .send()?
        .json()?;

    info!("details: {:?}", data);
    let details = data.details.get(&symbol.clone()).ok_or("Could not get detail from")?;

    Ok(details.clone())
}

#[cfg(test)]
mod tests {

    use super::*;
    use serde_json::json;

    #[test]
    fn deserialize_coindetails() {
        let json = json!({
        "data": {
            "BTC": {
                "id": 1,
                "name": "Bitcoin",
                "symbol": "BTC",
                "quote": {
                    "EUR": {
                        "price": 1.0,
                        "volume_24h": 2.0,
                    }
                }
            }
        }});
        let result: QuoteData = serde_json::from_value(json).unwrap();
        let quote: Quote = Quote {
            price: 1.0,
            volume_24h: 2.0,
        };
        let mut quotes: QuoteMap = HashMap::new();
        quotes.insert("EUR".to_string(), quote.clone());
        let detail: CoinDetail = CoinDetail {
            id: 1,
            name: "Bitcoin".to_string(),
            symbol: "BTC".to_string(),
            quotes,
        };
        let mut details: CoinDetailMap = HashMap::new();
        details.insert("BTC".to_string(), detail.clone());
        let expected: QuoteData = QuoteData { details };

        assert_eq!(result, expected)
    }

    fn mock_coin(id: i32) -> Coin {
        Coin {
            id,
            name: "any-coin".to_string(),
            symbol: "any-symbol".to_string(),
        }
    }

    #[test]
    fn coins_next() {
        let coin_a: Coin = mock_coin(0);
        let coin_b: Coin = mock_coin(1);
        let coin_c: Coin = mock_coin(2);
        let mut coins: Coins = Coins::new(vec![coin_a.clone(), coin_b.clone(), coin_c.clone()]);
        assert_eq!(coins.current(), Some(coin_a.clone()));
        coins.next();
        assert_eq!(coins.current(), Some(coin_b.clone()));
        coins.next();
        assert_eq!(coins.current(), Some(coin_c.clone()));
        coins.next();
        assert_eq!(coins.current(), Some(coin_a.clone()))
    }
    #[test]
    fn coins_prev() {
        let coin_a: Coin = mock_coin(0);
        let coin_b: Coin = mock_coin(1);
        let coin_c: Coin = mock_coin(2);
        let mut coins: Coins = Coins::new(vec![coin_a.clone(), coin_b.clone(), coin_c.clone()]);
        assert_eq!(coins.current(), Some(coin_a.clone()));
        coins.prev();
        assert_eq!(coins.current(), Some(coin_c.clone()));
        coins.prev();
        assert_eq!(coins.current(), Some(coin_b.clone()));
        coins.prev();
        assert_eq!(coins.current(), Some(coin_a.clone()))
    }
}
