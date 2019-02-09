
use reqwest::Url;
use std::collections::HashMap;

use super::Api;
use crate::app::{
    env::{get_env, ENV_COINMARKETCAP_KEY, HEADER_COINMARKETCAP_KEY},
    errors::AppError,
    types::{AppResult, Coin, Coins}
};


pub type CMCCoinList = Vec<CMCCoin>;

#[derive(Serialize, Deserialize, Debug)]
pub struct CMCCoinListData {
    #[serde(rename = "data")]
    pub coins: CMCCoinList,
}

pub type CMCCoinDetailMap = HashMap<String, CMCCoin>;

#[derive(Serialize, Debug, Deserialize, PartialEq)]
pub struct CMCQuoteData {
    #[serde(rename = "data")]
    pub details: CMCCoinDetailMap,
}

pub type CMCQuoteMap = HashMap<String, CMCQuote>;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CMCCoin {
    pub id: i32,
    pub name: String,
    pub symbol: String,
    #[serde(rename = "quote")]
    pub quotes: CMCQuoteMap,
}

#[derive(Serialize, Debug, Deserialize, Clone, PartialEq)]
pub struct CMCQuote {
    pub price: f32,
    pub volume_24h: f32,
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

    type ApiCoin = CMCCoin;

    fn get_endpoint(&self) -> &str {
        if self.is_development { 
            "http://localhost:3000"
        } else {
            "https://pro-api.coinmarketcap.com/v1/cryptocurrency"
        }
    }

    fn to_coin(&self, api_coin: &Self::ApiCoin, fiat: &str) -> Coin {
        Coin {
            name: api_coin.name.to_owned(),
            symbol: api_coin.symbol.to_owned(),
            quote: api_coin.quotes.get(fiat).map(|q| q.price),
        }
    }

    // fn get_coins(&self) -> AppResult<CoinList> {
    //         info!("fetch coins");
    //         // TODO: Parameterize "limit"
    //         let params = [("start", "1"), ("limit", "20")];
    //         let endpoint = format!("{}/map", self.get_endpoint());
    //         let url = Url::parse_with_params(&endpoint, &params).map_err(AppError::ApiParseUrl)?;
    //         let key = get_env(ENV_COINMARKETCAP_KEY)?

    //         info!("fetch coins url {}", url);

    //         self.client.get(url)
    //                 .header(HEADER_COINMARKETCAP_KEY, key)
    //                 .send()
    //                 .map_err(AppError::ApiRequest)?
    //                 .json()
    //                 .map_err(AppError::ApiRequest)
    //                 .map(|d: CoinListData| d.coins)
    // }

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

            let data: CMCQuoteData = self.client
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
                    }
                }
            }
        }});
        let result: CMCQuoteData = serde_json::from_value(json).unwrap();
        let quote: CMCQuote = CMCQuote {
            price: 1.0,
            volume_24h: 2.0,
        };
        let mut quotes: CMCQuoteMap = HashMap::new();
        quotes.insert("EUR".into(), quote.clone());
        let detail: CMCCoin = CMCCoin {
            id: 1,
            name: "Bitcoin".into(),
            symbol: "BTC".into(),
            quotes,
        };
        let mut details: CMCCoinDetailMap = HashMap::new();
        details.insert("BTC".into(), detail.clone());
        let expected: CMCQuoteData = CMCQuoteData { details };

        assert_eq!(result, expected)
    }

    #[test]
    fn to_coin() {
        let quote: CMCQuote = CMCQuote {
            price: 1.1,
            volume_24h: 2.2,
        };
        let mut quotes: CMCQuoteMap = HashMap::new();
        quotes.insert("EUR".into(), quote.clone());
        let api_coin: CMCCoin = CMCCoin {
            id: 1,
            name: "Bitcoin".into(),
            symbol: "BTC".into(),
            quotes,
        };

        let cmc = CoinMarketCap::new(false); 
        let result = cmc.to_coin(&api_coin, &"EUR"); 
        let expected: Coin = CoinBuilder::default()
                                .name("Bitcoin")
                                .symbol("BTC")
                                .quote(Some(1.1))
                                .build()
                                .unwrap();

        assert_eq!(result, expected);
    }
}
