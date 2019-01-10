mod api;
pub mod config;
pub mod constants;
pub mod env;
mod errors;
mod input;
mod types;

use self::api::{fetch_coins, fetch_detail};
use self::config::Config;
use self::errors::AppError;
use self::input::{InputChannel, InputEvent};
use self::types::{AppResult, AppTerminal, Coin, CoinDetail, CoinList, Coins};
use termion::event::Key;
use tui::style::{Color, Style};

extern crate failure;
extern crate termion;
extern crate tui;

use std::io;
use std::thread;
use std::time::Duration;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::layout::{Alignment, Constraint, Direction, Layout};
use tui::widgets::{Block, Borders, Paragraph, Tabs, Text, Widget};
use tui::Terminal;

pub struct App<'a> {
    #[derive(Debug)]
    env: Config<'a>,
    coins: Coins,
    coin_detail: Option<CoinDetail>,
    view_state: ViewState,
}

#[derive(Debug)]
pub enum ViewState {
    Welcome,
    List,
    Detail,
}

impl<'a> App<'a> {
    pub fn new(env: Config<'a>) -> Self {
        App {
            env,
            coins: Coins::default(),
            coin_detail: None,
            view_state: ViewState::Welcome,
        }
    }

    fn init_terminal(&self) -> AppResult<AppTerminal> {
        let stdout = io::stdout().into_raw_mode().map_err(AppError::Terminal)?;
        let stdout = AlternateScreen::from(stdout);
        let backend = TermionBackend::new(stdout);
        let mut terminal = Terminal::new(backend).map_err(AppError::Terminal)?;
        terminal.hide_cursor().map_err(AppError::Terminal)?;
        Ok(terminal)
    }

    fn get_current_coin(&self) -> Option<Coin> {
        self.coins.current()
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
        if let Some(coin) = &self.get_current_coin() {
            fetch_detail(&coin.symbol, &self.env.fiat_symbol)
        } else {
            Err(AppError::CurrentCoinMissing())
        }
    }

    fn render(&mut self, terminal: &mut AppTerminal) -> AppResult<()> {
        let size = terminal.size().map_err(AppError::Terminal)?;
        terminal
            .draw(|mut f| {
                Block::default()
                    .title("wtch-crpts")
                    .borders(Borders::ALL)
                    .render(&mut f, size);

                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(
                        [
                            Constraint::Percentage(10),
                            Constraint::Percentage(10),
                            Constraint::Min(0),
                        ]
                        .as_ref(),
                    )
                    .split(size);

                let block = Block::default().borders(Borders::NONE);
                match self.view_state {
                    ViewState::Welcome => {
                        // TODO: Create a factory to render a headline
                        Paragraph::new(vec![Text::raw("Welcome")].iter())
                            .block(block.clone())
                            .alignment(Alignment::Left)
                            .render(&mut f, chunks[1]);
                    }
                    ViewState::List => {
                        if self.coins.list.is_empty() {
                            Paragraph::new(vec![Text::raw("No coins available")].iter())
                                .block(block.clone())
                                .alignment(Alignment::Left)
                                .render(&mut f, chunks[1]);
                        } else {
                            Tabs::default()
                                .block(block.clone())
                                .titles(&self.coins.get_symbols())
                                .select(self.coins.index)
                                .style(Style::default().fg(Color::Cyan))
                                .highlight_style(Style::default().fg(Color::Yellow))
                                .render(&mut f, chunks[1]);
                            match &self.coin_detail {
                                Some(detail) => {
                                    Paragraph::new(
                                        vec![
                                            Text::raw(detail.name.as_str()),
                                            Text::raw(": "),
                                            Text::raw(detail.symbol.as_str()),
                                        ]
                                        .iter(),
                                    )
                                    .block(block.clone())
                                    .alignment(Alignment::Left)
                                    .render(&mut f, chunks[2]);
                                }
                                None => {
                                    Paragraph::new(vec![Text::raw("loading detail...")].iter())
                                        .block(block.clone())
                                        .alignment(Alignment::Left)
                                        .render(&mut f, chunks[2]);
                                }
                            }
                        }
                    }
                    ViewState::Detail => {
                        // TODO: Create a factory to render a headline
                        Paragraph::new(vec![Text::raw("Detail")].iter())
                            .block(block.clone())
                            .alignment(Alignment::Left)
                            .render(&mut f, chunks[1]);
                    }
                }
            })
            .map_err(AppError::Terminal)?;

        Ok(())
    }

    pub fn run(&mut self) -> AppResult<()> {
        let mut terminal = self.init_terminal()?;
        self.render(&mut terminal)?;
        let coins = self.get_coins()?;
        self.coins = Coins::new(coins);
        self.view_state = ViewState::List;
        self.render(&mut terminal)?;
        let detail = self.get_current_coin_detail()?;
        self.coin_detail = Some(detail);
        // info!("{:?}", self);

        let inp_channel = InputChannel::new();

        loop {
            self.render(&mut terminal)?;
            match inp_channel.rx.recv() {
                Ok(inp_event) => match inp_event {
                    InputEvent::Exit => {
                        break;
                    }
                    InputEvent::InputKey(key) => match key {
                        Key::Right => {
                            &self.coins.next();
                            let detail = self.get_current_coin_detail()?;
                            self.coin_detail = Some(detail);
                        }
                        Key::Left => {
                            &self.coins.prev();
                            let detail = self.get_current_coin_detail()?;
                            self.coin_detail = Some(detail);
                        }
                        _ => {}
                    },
                },
                Err(_) => eprintln!("Error to get InputEvent"),
            }
        }
        Ok(())
    }
}
