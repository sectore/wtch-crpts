extern crate clap;
extern crate dotenv;

use clap::{App, Arg};
use dotenv::dotenv;
use std::env;

extern crate env_logger;

extern crate wtch_crpts;
use wtch_crpts as app;

fn main() {
    env_logger::init();

    dotenv().ok();
    const ENV_COINMARKETCAP_KEY: &str = "COINMARKETCAP_KEY";
    let coinmarketcap_key = env::var(ENV_COINMARKETCAP_KEY.to_string()).map_err(|_| app::AppError::Env {
        name: ENV_COINMARKETCAP_KEY.to_string(),
    });

    let matches = App::new("WTCH-CRPTS")
        .about("Watch crypto's in your terminal")
        .author("jk <email@jkrause.io>")
        .version("0.1")
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
                .possible_values(&app::FIAT_LIST),
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
    let is_development = matches.value_of("dev").expect("is_development has to be set") == "true";
    let cryptos: Vec<&str> = matches
        .values_of("cryptocurrencies")
        .expect("One or more cryptocurrency has to be set")
        .collect();

    let app =
        coinmarketcap_key.map(|key| app::WatchCryptos::new(app::Env::new(key, cryptos, fiat, is_development)).run());

    match app {
        Ok(_) => (),
        Err(e) => eprintln!("Ooops, something went wrong to run the app: {}", e),
    }
}
