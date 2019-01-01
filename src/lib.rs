extern crate reqwest;
use reqwest::Url;

#[macro_use]
extern crate log;
extern crate env_logger;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

// For complete fiat list check:
// https://coinmarketcap.com/api/documentation/v1/#section/Standards-and-Conventions
pub const FIAT_LIST: [&str; 5] = ["USD", "EUR", "CNY", "RUB", "SGD"];

#[derive(Debug)]
pub struct Env<'a> {
    coinmarketcap_key: String,
    fiat_symbol: String,
    crypto_symbols: Vec<&'a str>,
    is_development: bool,
}

impl<'a> Env<'a> {
    pub fn new(coinmarketcap_key: &String, cryptos: &Vec<&'a str>, fiat: &str, is_development: bool) -> Self {
        Env {
            coinmarketcap_key: coinmarketcap_key.clone(),
            crypto_symbols: cryptos.to_vec(),
            fiat_symbol: fiat.to_string(),
            is_development,
        }
    }
}

#[derive(Debug)]
pub struct WatchCryptos<'a> {
    env: Env<'a>,
    cryptos: Option<Vec<Coin>>,
}

impl<'a> WatchCryptos<'a> {
    pub fn new(env: Env<'a>) -> Self {
        WatchCryptos { env, cryptos: None }
    }

    pub fn run(self) {
        let coins = fetch_coins(&self.env.coinmarketcap_key).expect("Could not fetch list of cryptocurrencies");
        let cryptos: Vec<Coin> = coins
            .into_iter()
            .filter(|coin| self.env.crypto_symbols.contains(&coin.symbol.as_str()))
            .collect();
        let s = self.set_cryptos(Some(cryptos));
        info!("{:?}", s.cryptos);
    }

    fn set_cryptos(mut self, cryptos: Option<Vec<Coin>>) -> Self {
        self.cryptos = cryptos;
        self
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Coin {
    id: f32,
    name: String,
    symbol: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListData {
    data: Vec<Coin>,
}

// const URL: &str = "https://pro-api.coinmarketcap.com/v1/cryptocurrency/listings/latest";
const URL_LATEST: &str = "http://localhost:3000/latest";
const URL_MAP: &str = "http://localhost:3000/map";

pub fn fetch_coins(key: &String) -> Result<Vec<Coin>, Box<std::error::Error>> {
    println!("latest:");

    let client = reqwest::Client::new();
    let url = Url::parse_with_params(URL_LATEST, &[("start", "1"), ("limit", "20"), ("convert", "USD")])?;
    let list: ListData = client
        .get(url)
        .header("X-CMC_PRO_API_KEY", key.clone())
        .send()?
        .json()?;

    Ok(list.data)
}

pub fn fetch_map(key: &String) -> Result<(), Box<std::error::Error>> {
    println!("map:");

    let client = reqwest::Client::new();
    let url = Url::parse_with_params(URL_LATEST, &[("start", "1"), ("limit", "20"), ("convert", "USD")])?;
    let body = client
        .get(url)
        .header("X-CMC_PRO_API_KEY", key.clone())
        .send()?
        .text()?;

    println!("body = {:?}", body);

    Ok(())
}
