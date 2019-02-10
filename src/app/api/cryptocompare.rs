
use reqwest::Url;
use std::collections::HashMap;

use super::Api;
use crate::app::{
    env::get_env,
    errors::AppError,
    types::{AppResult, Coins},
    types
};

const ENV_CRYPTOCOMPARE_KEY: &str = "CRYPTOCOMPARE_KEY";

type CoinMap = HashMap<String, QuoteMap>; 

#[derive(Serialize, Debug, Deserialize, PartialEq)]
struct CoinMapData {
    #[serde(rename = "RAW")]
    pub coins: CoinMap,
}

type QuoteMap = HashMap<String, Coin>; 

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Coin {
    #[serde(rename = "FROMSYMBOL")]
    pub symbol: String,
    #[serde(rename = "PRICE")]
    price: f32,
    #[serde(rename = "CHANGE24HOUR")]
    percent_change_24h: f32,
    #[serde(rename = "MKTCAP")]
    market_cap: f64,
}

pub struct CryptoCompare {
    client: reqwest::Client,
    is_development: bool,
}

impl CryptoCompare {
    pub fn new(is_development: bool) -> Self {
        CryptoCompare {
            client: reqwest::Client::new(),
            is_development
        }
    }

    fn to_coin(&self, api_coin: &Coin, _: &str) -> types::Coin {
        types::Coin {
            symbol: api_coin.symbol.to_owned(),
            quote: Some(api_coin.price),
            percent_change_24h: Some(api_coin.percent_change_24h),
            market_cap: Some(api_coin.market_cap),
        }
    }
}

impl Api for CryptoCompare {

    // type ApiCoin = Coin;

    fn get_endpoint(&self) -> &str {
        if self.is_development { 
            "http://localhost:3000/"
        } else {
            "https://min-api.cryptocompare.com/data/"
        }
    }

    fn get_coin_details(&self, symbols: &[&str], fiat: &str) -> AppResult<Coins> {
        let key: String = get_env(ENV_CRYPTOCOMPARE_KEY)?;
        let params = [("fsyms", symbols.join(",")), ("tsyms", fiat.into()), ("api_key", key)];
        let endpoint = if self.is_development {
                format!("{}/quotes", self.get_endpoint())
            } else {
                format!("{}/pricemultifull", self.get_endpoint())
            };
        let url = Url::parse_with_params(&endpoint, &params).map_err(AppError::ApiParseUrl)?;
        info!("fetch detail url {}", url);
        
        let data: CoinMapData = self.client
                .get(url)
                .send()
                .map_err(AppError::ApiRequest)?
                .json()
                .map_err(AppError::ApiRequest)?;

        let mut coin_list = Vec::new();       
        for symbol in symbols {
            let s = *symbol;
            let result = data.coins.get(s)
                            .ok_or(AppError::ApiParseMap { key: String::from(s) });

            info!("result {:?}", result);
            // TODO: Show not found symbols in UI
            // but for now just ingore those
            if let Ok(quotes) = result {
                let coin = quotes.get(fiat)
                                .ok_or(AppError::ApiParseMap { key: String::from(fiat) });
                if let Ok(c) = coin {
                    coin_list.push(self.to_coin(c, &fiat));
                }
            };
        };
        info!("details {:?}", coin_list);
        Ok(Coins::new(coin_list))
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use serde_json::json;

    #[test]
    fn deserialize_coins() {
        let json = json!({
            "RAW": {
                "BTC": {
                    "EUR": {
                        "TYPE": "5",
                        "MARKET": "CCCAGG",
                        "FROMSYMBOL": "BTC",
                        "TOSYMBOL": "EUR",
                        "PRICE": 3200.98,
                        "CHANGE24HOUR": -25.71,
                        "MKTCAP": 56_110_256_905.26,
                    }
                }
            }
        });
        let result: CoinMapData = serde_json::from_value(json).unwrap();
        
        let coin: Coin = Coin {
            symbol: "BTC".into(),
            price: 3200.98,
            percent_change_24h: -25.71,
            market_cap: 56_110_256_905.26,
        };
        let mut quotes: QuoteMap = HashMap::new();
        quotes.insert("EUR".into(), coin.clone());

        let mut coins: CoinMap = HashMap::new();
        coins.insert("BTC".into(), quotes.clone());

        let expected: CoinMapData = CoinMapData { coins };

        assert_eq!(result, expected)
    }
}
