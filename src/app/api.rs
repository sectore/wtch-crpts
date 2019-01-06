use reqwest::Url;

use super::constants::{URL_MAP, URL_QUOTES};
use super::env::{get_env, ENV_COINMARKETCAP_KEY, HEADER_COINMARKETCAP_KEY};
use super::errors::AppError;
use super::types::{AppResult, CoinDetail, CoinList, CoinListData, QuoteData};

pub fn fetch_coins() -> AppResult<CoinList> {
        info!("fetch coins");

        let client = reqwest::Client::new();
        // TODO: Parameterize "limit"
        let params = [("start", "1"), ("limit", "5000")];
        let url = Url::parse_with_params(URL_MAP, &params).map_err(AppError::ApiParseUrl)?;
        let key = get_env(ENV_COINMARKETCAP_KEY)?;

        info!("fetch coins url {}", url);

        client.get(url)
                .header(HEADER_COINMARKETCAP_KEY, key)
                .send()
                .map_err(AppError::ApiRequest)?
                .json()
                .map_err(AppError::ApiRequest)
                .map(|d: CoinListData| d.coins)
}

pub fn fetch_detail(symbol: &String, fiat: &str) -> AppResult<CoinDetail> {
        let client = reqwest::Client::new();
        let ref fiat_r = fiat.into();
        let params = [("symbol", &symbol), ("convert", &fiat_r)];
        let url = Url::parse_with_params(URL_QUOTES, &params).map_err(AppError::ApiParseUrl)?;
        let key: String = get_env(ENV_COINMARKETCAP_KEY)?;

        info!("fetch detail url {}", url);

        let data: QuoteData = client
                .get(url)
                .header(HEADER_COINMARKETCAP_KEY, key.clone())
                .send()
                .map_err(AppError::ApiRequest)?
                .json()
                .map_err(AppError::ApiRequest)?;

        data.details
                .get(&symbol.clone())
                .map(|detail| detail.to_owned())
                .ok_or(AppError::ApiParseMap { key: symbol.clone() })
}
