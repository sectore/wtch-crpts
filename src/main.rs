#[macro_use]
extern crate clap;
extern crate dotenv;
extern crate wtch_crpts;

use clap::{App, Arg};
use wtch_crpts as wc;

fn main() {
    env_logger::init();

    let matches = App::new("WTCH-CRPTS")
        .about(crate_description!())
        .author(crate_authors!())
        .version(crate_version!())
        .args(&[
            Arg::with_name("cryptocurrencies")
                .help("Cryptocurrency to watch, e.g. BTC. Multiple values can be added by using ',' as a delimiter, eg. BTC,ETH,LTC")
                .short("c")
                .long("cryptos")
                .use_delimiter(true)
                .multiple(true)
                .default_value("BTC"),
            Arg::with_name("fiat")
                .help("Fiat currency for rating cryptocurrencies, e.g. EUR")
                .short("f")
                .long("fiat")
                .default_value("USD")
                .possible_values(&wc::app::constants::FIAT_LIST),
            Arg::with_name("dev")
                .help("Fiat currency for rating cryptocurrencies, e.g. EUR")
                .short("d")
                .long("development")
                .default_value("false")
                .hidden_short_help(true)
                .hidden_long_help(true),
        ])
        .get_matches();

    let fiat = matches.value_of("fiat").expect("fiat has to be set");
    let is_development = matches.occurrences_of("dev") == 1;
    let cryptos: Vec<&str> = matches
        .values_of("cryptocurrencies")
        .expect("One or more cryptocurrency has to be set")
        .collect();

    let mut app = wc::WatchCryptos::new(wc::app::env::Env::new(cryptos, fiat, is_development));
    match app.run() {
        Ok(_) => println!("App is running successfully ..."),
        Err(e) => eprintln!("Ooops, something went wrong: {}", e),
    }
}
