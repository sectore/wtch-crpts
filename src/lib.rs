extern crate reqwest;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate derive_builder;
#[macro_use]
extern crate failure;

pub mod app;
use self::app::api::{fetch_coins, fetch_detail};
use self::app::env::Env;
use self::app::errors::AppError;
use self::app::types::{AppResult, Coin, CoinDetail, CoinList, Coins};

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

    pub fn run(&mut self) -> AppResult<()> {
        let coins = self.get_coins()?;
        self.coins = Some(Coins::new(coins));
        let detail = self.get_current_coin_detail()?;
        self.coin_detail = Some(detail);
        info!("{:?}", self);
        Ok(())
    }

    fn current_coin(&self) -> Option<Coin> {
        self.coins.as_ref().and_then(|cs| cs.current())
    }

    fn get_coins(&mut self) -> AppResult<CoinList> {
        let result = fetch_coins()?;
        let coins: CoinList = result
            .into_iter()
            .filter(|coin| self.env.crypto_symbols.contains(&coin.symbol.as_str()))
            .collect();
        if coins.is_empty() {
            // Paaaanic.... Just because we do need at least one supported crypto to run the app
            panic!(format!("Cryptocurrencies {:?} are not supported", coins))
        } else {
            Ok(coins)
        }
    }

    fn get_current_coin_detail(&mut self) -> AppResult<CoinDetail> {
        if let Some(coin) = &self.current_coin() {
            fetch_detail(&coin.symbol, &self.env.fiat_symbol)
        } else {
            Err(AppError::CurrentCoinMissing())
        }
    }
}
