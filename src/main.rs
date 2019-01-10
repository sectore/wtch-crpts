#[macro_use]
extern crate clap;
extern crate dotenv;

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

mod app;

use clap::{App, Arg};

fn main() {
    env_logger::init();

    let matches = App::new(crate_name!())
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
                .possible_values(&app::constants::FIAT_LIST),
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

    let mut app = app::App::new(app::config::Config::new(cryptos, fiat, is_development));
    let msg = match app.run() {
        Ok(_) => String::from("Goodbye !"),
        Err(e) => format!("Ooops, something went wrong: {}", e),
    };
    println!("{}", msg);
}
