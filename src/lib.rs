extern crate reqwest;
use reqwest::Url;

#[macro_use]
extern crate log;
extern crate env_logger;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate derive_builder;

#[macro_use]
extern crate failure;

use std::collections::HashMap;
use std::error::Error;

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

#[derive(Debug, Fail)]
pub enum AppError {
    #[fail(display = "Failed to get envoirenment variable {}", name)]
    Env { name: String },
    #[fail(display = "serde error")]
    SerdeError(#[cause] serde_json::Error),
    #[fail(display = "Request to Api failed {:?}", _0)]
    ApiRequest(#[cause] reqwest::Error),
    #[fail(display = "Parsing url failed {:?}", _0)]
    ApiParseUrl(#[cause] reqwest::UrlError),
    #[fail(display = "Failed to parse value of {} from list", key)]
    ApiParseMap { key: String },
    #[fail(display = "Current coin does not exist")]
    CurrentCoinMissing(),
}

impl From<serde_json::Error> for AppError {
    fn from(e: serde_json::Error) -> Self {
        AppError::SerdeError(e)
    }
}

impl From<reqwest::Error> for AppError {
    fn from(e: reqwest::Error) -> Self {
        AppError::ApiRequest(e)
    }
}

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

pub type CoinList = Vec<Coin>;

#[derive(Debug)]
struct Coins {
    index: usize,
    list: CoinList,
}

impl Coins {
    fn new(list: CoinList) -> Self {
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

    pub fn run(&mut self) -> Result<(), AppError> {
        let coins = self.get_coins()?;
        info!("run {:?} coins", &coins);
        self.coins = Some(Coins::new(coins));
        let detail = self.get_current_coin_detail()?;
        info!("run {:?} detail", &detail);
        self.coin_detail = Some(detail);

        info!("{:?}", self);

        Ok(())
    }

    fn current_coin(&self) -> Option<Coin> {
        self.coins.as_ref().and_then(|cs| cs.current())
    }

    fn get_coins(&mut self) -> Result<CoinList, AppError> {
        let result = fetch_coins(&self.env.coinmarketcap_key)?;
        let coins: CoinList = result
            .into_iter()
            .filter(|coin| self.env.crypto_symbols.contains(&coin.symbol.as_str()))
            .collect();

        info!("coins {:?}", &coins);
        if coins.is_empty() {
            // Paaaanic.... Just because we do need at least one supported crypto to run the app
            panic!(format!("Cryptocurrencies {:?} are not supported", coins))
        } else {
            Ok(coins)
        }
    }

    fn get_current_coin_detail(&mut self) -> Result<CoinDetail, AppError> {
        if let Some(coin) = &self.current_coin() {
            fetch_detail(&self.env.coinmarketcap_key, &coin.symbol)
        } else {
            Err(AppError::CurrentCoinMissing())
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Builder, Default)]
#[builder(default, setter(into))]
pub struct Coin {
    id: i32,
    name: String,
    #[builder(setter(into))]
    symbol: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CoinListData {
    #[serde(rename = "data")]
    coins: CoinList,
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

pub type ApiError = Box<Error>;
pub type ApiResult<T> = Result<T, ApiError>;

pub fn fetch_coins(key: &String) -> Result<CoinList, AppError> {
    info!("fetch coins");

    let client = reqwest::Client::new();
    let params = [("start", "1"), ("limit", "5000")];
    let url = Url::parse_with_params(URL_MAP, &params).map_err(AppError::ApiParseUrl)?;
    info!("url {:?}", &url);

    client
        .get(url)
        .header("X-CMC_PRO_API_KEY", key.clone())
        .send()
        .map_err(AppError::ApiRequest)?
        .json()
        .map_err(AppError::ApiRequest)
        .map(|d: CoinListData| d.coins)
}

// const URL_QUOTES: &str = "https://pro-api.coinmarketcap.com/v1/cryptocurrency/quotes/latest";
const URL_QUOTES: &str = "http://localhost:3000/quotes";

pub fn fetch_detail(key: &String, symbol: &String) -> Result<CoinDetail, AppError> {
    info!("fetch detail");

    let client = reqwest::Client::new();
    let params = [("start", "1"), ("limit", "5000")];
    let url = Url::parse_with_params(URL_QUOTES, &params).map_err(AppError::ApiParseUrl)?;
    info!("url2 {:?}", &url);

    let data: QuoteData = client
        .get(url)
        .header("X-CMC_PRO_API_KEY", key.clone())
        .send()
        .map_err(AppError::ApiRequest)?
        .json()
        .map_err(AppError::ApiRequest)?;

    data.details
        .get(&symbol.clone())
        .ok_or(AppError::ApiParseMap { key: symbol.clone() })
        // Clone detail to be "dereferenced"
        .map(|d| d.clone())
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

    #[test]
    fn coins_next() {
        let coin_a: Coin = CoinBuilder::default().id(0).build().unwrap();
        let coin_b: Coin = CoinBuilder::default().id(1).build().unwrap();
        let coin_c: Coin = CoinBuilder::default().id(2).build().unwrap();
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
        let coin_a: Coin = CoinBuilder::default().id(0).build().unwrap();
        let coin_b: Coin = CoinBuilder::default().id(1).build().unwrap();
        let coin_c: Coin = CoinBuilder::default().id(2).build().unwrap();
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
