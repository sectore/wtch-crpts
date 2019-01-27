pub mod api;
pub mod config;
pub mod constants;
pub mod env;
mod errors;
mod input;
mod types;

use self::{
    api::{Api},
    config::Config,
    errors::AppError,
    input::{InputChannel, InputEvent},
    types::{AppResult, AppTerminal, Coins},
};

use termion::event::Key;
use tui::{
    style::{Color, Style},
    widgets::{Table, Row},
};

extern crate failure;
extern crate termion;
extern crate tui;

use std::io;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::layout::{Alignment, Constraint, Direction, Layout};
use tui::widgets::{Block, Borders, Paragraph, Text, Widget};
use tui::Terminal;

pub struct App<'a, T> {
    config: Config<'a, T>,
    coins: Option<Coins>,
    view_state: ViewState,
}

#[derive(Debug)]
pub enum ViewState {
    Welcome,
    List,
}

impl<'a, T: Api> App<'a, T> {
    pub fn new(config: Config<'a, T>) -> Self {
        App {
            config,
            coins: None,
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

    // fn get_coins(&mut self) -> AppResult<CoinList> {
    //     let result = self.config.api.get_coins()?;
    //     let coins: CoinList = result
    //         .into_iter()
    //         .filter(|coin| self.config.crypto_symbols.contains(&coin.symbol.as_str()))
    //         .collect();
    //     if coins.is_empty() {
    //         // Paaaanic.... We do need at least one supported crypto to run the app
    //         panic!(format!("Cryptocurrencies {:?} are not supported", coins))
    //     } else {
    //         Ok(coins)
    //     }
    // }

    fn render(&mut self, terminal: &mut AppTerminal) -> AppResult<()> {
        let size = terminal.size().map_err(AppError::Terminal)?;
        terminal
            .draw(|mut f| {
                Block::default()
                    .title("wtch-crpts")
                    .borders(Borders::ALL)
                    .render(&mut f, size);

                let rects = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(
                        [
                            Constraint::Percentage(90),
                            Constraint::Percentage(10),
                        ]
                        .as_ref(),
                    )
                    .margin(2)
                    .split(size);

                let block = Block::default().borders(Borders::NONE);
                match self.view_state {
                    ViewState::Welcome => {
                        // TODO: Create a factory to render a headline
                        Paragraph::new(vec![Text::raw("Welcome")].iter())
                            .block(block.clone())
                            .alignment(Alignment::Left)
                            .render(&mut f, rects[0]);
                    }
                    ViewState::List => {
                        if let Some(coins) = &self.coins {
                            let mut rows = Vec::new();
                            let normal_style =  Style::default().fg(Color::Black);
                            let selected_style =  Style::default().fg(Color::Yellow);
                            for coin in &coins.list {
                                let quote = match &coin.quote {
                                    None => "-".into(),
                                    Some(q) => q.to_string()
                                };
                                let name = &coin.name;
                                let symbol = &coin.symbol;
                                let row = vec![name.clone(), symbol.clone(), quote.clone()].into_iter();
                                let style = match coins.current() {
                                    Some(current) => if current.symbol == coin.symbol {
                                        selected_style
                                    } else {
                                        normal_style
                                    },
                                    None => normal_style
                                };
                                rows.push(Row::StyledData(row, style));
                            };

                            Table::new(
                                ["coin", "symbol", self.config.fiat_symbol].into_iter(),
                                rows.into_iter()
                            )
                            .block(Block::default().borders(Borders::NONE))
                            .column_spacing(1)
                            .widths(&[10, 10, 10])
                            .render(&mut f, rects[0]);
                        }
                    }
                }
            })
            .map_err(AppError::Terminal)?;

        Ok(())
    }

    pub fn run(&mut self) -> AppResult<()> {
        let mut terminal = self.init_terminal()?;
        // self.render(&mut terminal)?;
        // let coins = self.get_coins()?;
        // self.coins = Coins::new(coins);
        self.view_state = ViewState::List;
        self.render(&mut terminal)?;
        let coins = self.config.api.get_coin_details(&self.config.crypto_symbols, &self.config.fiat_symbol)?;
        self.coins = Some(coins);
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
                        Key::Up => {
                            if let Some(coins) = &mut self.coins {
                                &coins.next();
                                ()
                            }
                        }
                        Key::Down => {
                            if let Some(coins) = &mut self.coins {
                                &coins.prev();
                                ()
                            }
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
