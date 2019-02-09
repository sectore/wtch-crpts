
use reqwest::Url;
use std::collections::HashMap;

use super::Api;
use crate::app::{
    env::{get_env},
    errors::AppError,
    types::{AppResult, Coins},
    types
};


const HEADER_COINMARKETCAP_KEY: &str = "X-CMC_PRO_API_KEY";
const ENV_COINMARKETCAP_KEY: &str = "COINMARKETCAP_KEY";

type CoinDetailMap = HashMap<String, Coin>;

#[derive(Serialize, Debug, Deserialize, PartialEq)]
struct QuoteData {
    #[serde(rename = "data")]
    pub details: CoinDetailMap,
}

type QuoteMap = HashMap<String, Quote>;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Coin {
    pub id: i32,
    pub name: String,
    pub symbol: String,
    #[serde(rename = "quote")]
    quotes: QuoteMap,
}

#[derive(Serialize, Debug, Deserialize, Clone, PartialEq)]
struct Quote {
    pub price: f32,
    pub volume_24h: f32,
    pub percent_change_24h: f32,
    pub market_cap: f32,
}

pub struct CoinMarketCap {
    client: reqwest::Client,
    is_development: bool,
}

impl CoinMarketCap {
    pub fn new(is_development: bool) -> Self {
        CoinMarketCap {
            client: reqwest::Client::new(),
            is_development
        }
    }
}

impl Api for CoinMarketCap {

    type ApiCoin = Coin;

    fn get_endpoint(&self) -> &str {
        if self.is_development { 
            "http://localhost:3000"
        } else {
            "https://pro-api.coinmarketcap.com/v1/cryptocurrency"
        }
    }

    fn to_coin(&self, api_coin: &Self::ApiCoin, fiat: &str) -> types::Coin {
        types::Coin {
            symbol: api_coin.symbol.to_owned(),
            quote: api_coin.quotes.get(fiat).map(|q| q.price),
            percent_change_24h: api_coin.quotes.get(fiat).map(|q| q.percent_change_24h),
            market_cap: api_coin.quotes.get(fiat).map(|q| q.market_cap),
        }
    }

    fn get_coin_details(&self, symbols: &[&str], fiat: &str) -> AppResult<Coins> {
            let params = [("symbol", symbols.join(",")), ("convert", fiat.into())];
            let endpoint = if self.is_development {
                    format!("{}/quotes", self.get_endpoint())
                } else {
                    format!("{}/quotes/latest", self.get_endpoint())
                };
            let url = Url::parse_with_params(&endpoint, &params).map_err(AppError::ApiParseUrl)?;
            let key: String = get_env(ENV_COINMARKETCAP_KEY)?;

            info!("fetch detail url {}", url);

            let data: QuoteData = self.client
                    .get(url)
                    .header(HEADER_COINMARKETCAP_KEY, key.to_owned())
                    .send()
                    .map_err(AppError::ApiRequest)?
                    .json()
                    .map_err(AppError::ApiRequest)?;

            let mut coin_list = Vec::new();

            for symbol in symbols {
                let s = *symbol;
                let result = data.details.get(s)
                                .ok_or(AppError::ApiParseMap { key: String::from(s) });

                info!("result {:?}", result);
                // TODO: Show not found symbols in UI
                // but for now just ingore those
                if let Ok(cmc_coin) = result {
                    coin_list.push(self.to_coin(cmc_coin, &fiat));
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
    use crate::app::types::{CoinBuilder};
    use crate::app::types;

    #[test]
    fn deserialize_cmc_coins() {
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
                        "percent_change_24h": 50.0,
                        "market_cap": 200.0,
                    }
                }
            }
        }});
        let result: QuoteData = serde_json::from_value(json).unwrap();
        let quote: Quote = Quote {
            price: 1.0,
            volume_24h: 2.0,
            percent_change_24h: 50.0,
            market_cap: 200.0,
        };
        let mut quotes: QuoteMap = HashMap::new();
        quotes.insert("EUR".into(), quote.clone());
        let detail: Coin = Coin {
            id: 1,
            name: "Bitcoin".into(),
            symbol: "BTC".into(),
            quotes,
        };
        let mut details: CoinDetailMap = HashMap::new();
        details.insert("BTC".into(), detail.clone());
        let expected: QuoteData = QuoteData { details };

        assert_eq!(result, expected)
    }

    #[test]
    fn to_coin() {
        let quote: Quote = Quote {
            price: 1.1,
            volume_24h: 2.2,
            percent_change_24h: 55.0,
            market_cap: 222.0,
        };
        let mut quotes: QuoteMap = HashMap::new();
        quotes.insert("EUR".into(), quote.clone());
        let api_coin: Coin = Coin {
            id: 1,
            name: "Bitcoin".into(),
            symbol: "BTC".into(),
            quotes,
        };

        let cmc = CoinMarketCap::new(false); 
        let result = cmc.to_coin(&api_coin, &"EUR"); 
        let expected: types::Coin = CoinBuilder::default()
                                .symbol("BTC")
                                .quote(Some(1.1))
                                .percent_change_24h(Some(55.0))
                                .market_cap(Some(222.0))
                                .build()
                                .unwrap();

        assert_eq!(result, expected);
    }
}
