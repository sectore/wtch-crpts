extern crate reqwest;
use reqwest::Url;

#[macro_use]
extern crate log;
extern crate env_logger;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

use serde::Deserialize;
use serde::Deserializer;
use serde_json::{from_value, Value};

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
        self.index = if self.index + 1 >= self.list.len() {
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
                let result = fetch_quote(
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Coin {
    id: i32,
    name: String,
    symbol: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CoinDetail {
    id: i32,
    name: String,
    symbol: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListData {
    data: Vec<Coin>,
}

// const URL_MAP: &str = "https://pro-api.coinmarketcap.com/v1/cryptocurrency/map";
const URL_MAP: &str = "http://localhost:3000/map";

pub fn fetch_coins(key: &String) -> Result<Vec<Coin>, Box<std::error::Error>> {
    info!("coins:");

    let client = reqwest::Client::new();
    let url = Url::parse_with_params(URL_MAP, &[("start", "1"), ("limit", "5000")])?;
    let list: ListData = client
        .get(url)
        .header("X-CMC_PRO_API_KEY", key.clone())
        .send()?
        .json()?;

    Ok(list.data)
}

// const URL_QUOTES: &str = "https://pro-api.coinmarketcap.com/v1/cryptocurrency/quotes/latest";
const URL_QUOTES: &str = "http://localhost:3000/quotes";

#[derive(Serialize, Debug, Deserialize)]
pub struct QuoteData {
    #[serde(rename = "data", deserialize_with = "deserialize_coindetail")]
    detail: CoinDetail,
    // TODO(sectore) Deserialize quote
    // quote: Qutoe
}

pub fn fetch_quote(key: &String, symbol: &String, fiat: &String) -> Result<CoinDetail, Box<std::error::Error>> {
    info!("quote:");

    let client = reqwest::Client::new();
    let url = Url::parse_with_params(URL_QUOTES, &[("symbol", &symbol), ("convert", &fiat)])?;
    let list: QuoteData = client
        .get(url)
        .header("X-CMC_PRO_API_KEY", key.clone())
        .send()?
        .json()?;

    info!("list: {:?}", list);
    Ok(list.detail)
}

fn deserialize_coindetail<'de, D>(d: D) -> Result<CoinDetail, D::Error>
where
    D: Deserializer<'de>,
{
    let json: Value = Deserialize::deserialize(d)?;
    // We don't know the key of the map (it can be "BTC" or any other crypto symbol),
    // but we do know it's the first value we are interested in
    if let Some(detail) = json.as_object().unwrap().values().nth(0) {
        Ok(from_value(detail.clone()).unwrap())
    } else {
        Err(serde::de::Error::custom(format!(
            "Error to parse crypto detail from {}",
            json
        )))
    }
}
