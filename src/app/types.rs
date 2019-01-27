use std::io::Stdout;
use termion::raw::RawTerminal;
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::Terminal;

use super::errors::AppError;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Builder, Default)]
#[builder(default, setter(into))]
pub struct Coin {
    pub name: String,
    #[builder(setter(into))]
    pub symbol: String,
    pub quote: Option<f32>,
}

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

    #[allow(dead_code)]
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

pub type AppResult<T> = Result<T, AppError>;

pub type AppTerminalBackend = TermionBackend<AlternateScreen<RawTerminal<Stdout>>>;
pub type AppTerminal = Terminal<AppTerminalBackend>;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn coins_next() {
        let coin_a: Coin = CoinBuilder::default().name("a").build().unwrap();
        let coin_b: Coin = CoinBuilder::default().name("b").build().unwrap();
        let coin_c: Coin = CoinBuilder::default().name("c").build().unwrap();
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
        let coin_a: Coin = CoinBuilder::default().name("a").build().unwrap();
        let coin_b: Coin = CoinBuilder::default().name("b").build().unwrap();
        let coin_c: Coin = CoinBuilder::default().name("c").build().unwrap();
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
