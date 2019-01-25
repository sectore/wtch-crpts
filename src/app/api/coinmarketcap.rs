
use reqwest::Url;

use super::{
    Api,
    super::{
        env::{get_env, ENV_COINMARKETCAP_KEY, HEADER_COINMARKETCAP_KEY},
        errors::AppError,
        types::{AppResult, CoinDetail, CoinList, CoinListData, QuoteData}
    }
};

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

    fn get_endpoint(&self) -> &str {
        if self.is_development { 
            "http://localhost:3000"
        } else {
            "https://pro-api.coinmarketcap.com/v1/cryptocurrency"
        }
    }

    fn get_coins(&self) -> AppResult<CoinList> {
            info!("fetch coins");
            // TODO: Parameterize "limit"
            let params = [("start", "1"), ("limit", "20")];
            let endpoint = format!("{}/map", self.get_endpoint());
            let url = Url::parse_with_params(&endpoint, &params).map_err(AppError::ApiParseUrl)?;
            let key = get_env(ENV_COINMARKETCAP_KEY)?;

            info!("fetch coins url {}", url);

            self.client.get(url)
                    .header(HEADER_COINMARKETCAP_KEY, key)
                    .send()
                    .map_err(AppError::ApiRequest)?
                    .json()
                    .map_err(AppError::ApiRequest)
                    .map(|d: CoinListData| d.coins)
    }

    fn get_coin_detail(&self, symbol: &String, fiat: &str) -> AppResult<CoinDetail> {
            let ref fiat_r = fiat.into();
            let params = [("symbol", &symbol), ("convert", &fiat_r)];
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

            data.details
                    .get(&symbol.clone())
                    .map(|detail| detail.to_owned())
                    .ok_or(AppError::ApiParseMap { key: symbol.to_owned() })
    }
}