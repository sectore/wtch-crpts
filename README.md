# wtch-crpts (wɒtʃ ˈkrɪptəʊz) ~~~ >>> ~~~ `watch cryptos`

`wtch-crpts` is a _**personal | funny | playground | [Rust](https://www.rust-lang.org/) | thing**_ I started at the holidays back in December 2018. Its goal was just to dive into [Rust](https://www.rust-lang.org/) by covering _**common Rust stuff and libraries**_ - that's why the code might not make sense in all cases...

_**common stuff**_

- [Ownership, References, Borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
- [Generics, Traits, Lifetimes](https://doc.rust-lang.org/book/ch10-00-generics.html)
- [Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [Testing](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Concurrency, Threads](https://doc.rust-lang.org/book/ch16-01-threads.html)
- [Builder pattern](https://github.com/rust-unofficial/patterns/blob/master/patterns/builder.md)
- etc.

_**libs**_

- [fail](https://crates.io/crates/fail) - A fail point implementation for Rust.
- [dotenv](https://crates.io/crates/dotenv) - A `dotenv` implementation for Rust
- [clap](https://crates.io/crates/clap) - Command Line Argument Parser for Rust
- [tui](https://crates.io/crates/tui) A library to build rich terminal user interfaces or dashboards 
- [derive_builder](https://crates.io/crates/derive_builder) Rust macro to automatically implement the builder pattern for arbitrary structs.
- [serde](https://crates.io/crates/serde) - Serializing and deserializing Rust data structures efficiently and generically
- etc.

## Requirements

- [rustup](https://www.rust-lang.org/tools/install)

## How to run?

- Make a copy of `.env.example` and rename it to `.env`
- Get your [CoinMarketCap API key](https://coinmarketcap.com/api/) and add it to `COINMARKETCAP_KEY` in `.env`
- Build sources
```sh
cargo build
```

* Show help
```
./target/debug/wtch-crpts --help

WTCH-CRPTS 0.1.0
jk <email@jkrause.io>
Watch crypto's in your terminal

USAGE:
    wtch-crpts [OPTIONS]

FLAGS:
    -h, --help       
            Prints help information

    -V, --version    
            Prints version information


OPTIONS:
    -c, --cryptos <cryptocurrencies>...    
            Cryptocurrency to watch, e.g. BTC. Multiple values can be added by using ',' as a delimiter, eg. BTC,ETH,LTC
            [default: BTC]
    -f, --fiat <fiat>                      
            Fiat currency for rating cryptocurrencies, e.g. EUR [default: USD]  [possible values: USD, ALL, DZD, ARS,
            AMD, AUD, AZN, BHD, BDT, BYN, BMD, BOB, BAM, BRL, BGN, KHR, CAD, CLP, CNY, COP, CRC, HRK, CUP, CZK, DKK,
            DOP, EGP, EUR, GEL, GHS, GTQ, HNL, HKD, HUF, ISK, INR, IDR, IRR, IQD, ILS, JMD, JPY, JOD, KZT, KES, KWD,
            KGS, LBP, MKD, MYR, MUR, MXN, MDL, MNT, MAD, MMK, NAD, NPR, TWD, NDZ, NIO, NGN, NOK, OMR, PKR, PAB, PEN,
            PHP, PLN, GBP, QAR, RON, RUB, SAR, RSD, SGD, ZAR, KRW, SSP, VES, LKR, SEK, CHF, THB, TTD, TND, TRY, UGX,
            UAH, AED, UYU, UZS, VND]

```
* Run (e.g. to get rates of `BTC`, `ETH`, `LTC` in `EUR`)
```sh
./target/debug/wtch-crpts -f EUR -c=BTC,ETH,LTC
```
- Run with logging
```sh
RUST_LOG=wtch_crpts=debug ./target/debug/wtch-crpts -f EUR -c=BTC,ETH,LTC
```

- Run with mock data to save [your credits](https://pro.coinmarketcap.com/account) while doing some development. To serve mock data installation of [`json-server`](https://github.com/typicode/json-server) is required:
```sh
json-server -w mock/coinmarketcap-api.json
RUST_LOG=wtch_crpts=debug ./target/debug/wtch-crpts -f EUR -c=BTC,ETH,LTC
```

- Run tests
```
cargo test
```