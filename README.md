## ... _still WIP_ ...

# wtch-crpts (wɒtʃ ˈkrɪptəʊz) 

## _or just `watch cryptos`_

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