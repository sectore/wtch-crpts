use std::collections::HashMap;
use std::io::Stdout;
use termion::raw::RawTerminal;
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::Terminal;

use super::errors::AppError;

pub type CoinList = Vec<Coin>;

#[derive(Debug)]
pub struct Coins {
    pub index: usize,
    pub list: CoinList,
}

impl Coins {
    pub fn new(list: CoinList) -> Self {
        Coins { list, index: 0 }
    }

    pub fn current(&self) -> Option<Coin> {
        self.list.get(self.index).map(|x| x.clone())
    }

    #[allow(dead_code)]
    pub fn prev(&mut self) -> Option<Coin> {
        self.index = if self.index >= 1 {
            self.index - 1
        } else {
            self.list.len() - 1
        };
        self.current()
    }

    pub fn get_symbols(&self) -> Vec<String> {
        self.list.clone().into_iter().map(|coin| coin.symbol).collect()
    }
}

impl Default for Coins {
    fn default() -> Coins {
        Coins { index: 0, list: vec![] }
    }
}

impl Iterator for Coins {
    type Item = Coin;
    fn next(&mut self) -> Option<Coin> {
        self.index = (self.index + 1) % self.list.len();
        self.current()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Builder, Default)]
#[builder(default, setter(into))]
pub struct Coin {
    pub id: i32,
    pub name: String,
    #[builder(setter(into))]
    pub symbol: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CoinListData {
    #[serde(rename = "data")]
    pub coins: CoinList,
}

pub type CoinDetailMap = HashMap<String, CoinDetail>;

#[derive(Serialize, Debug, Deserialize, PartialEq)]
pub struct QuoteData {
    #[serde(rename = "data")]
    pub details: CoinDetailMap,
}

pub type QuoteMap = HashMap<String, Quote>;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CoinDetail {
    pub id: i32,
    pub name: String,
    pub symbol: String,
    #[serde(rename = "quote")]
    pub quotes: QuoteMap,
}

#[derive(Serialize, Debug, Deserialize, Clone, PartialEq)]
pub struct Quote {
    pub price: f32,
    pub volume_24h: f32,
}

pub type AppResult<T> = Result<T, AppError>;

pub type AppTerminalBackend = TermionBackend<AlternateScreen<RawTerminal<Stdout>>>;
pub type AppTerminal = Terminal<AppTerminalBackend>;

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
        quotes.insert("EUR".into(), quote.clone());
        let detail: CoinDetail = CoinDetail {
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
