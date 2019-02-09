// FIAT list supported by coinmarketcap
// https://coinmarketcap.com/api/documentation/v1/#section/Standards-and-Conventions
pub const FIAT_LIST: [&str; 93] = [
    "USD", "ALL", "DZD", "ARS", "AMD", "AUD", "AZN", "BHD", "BDT", "BYN", "BMD", "BOB", "BAM", "BRL", "BGN", "KHR",
    "CAD", "CLP", "CNY", "COP", "CRC", "HRK", "CUP", "CZK", "DKK", "DOP", "EGP", "EUR", "GEL", "GHS", "GTQ", "HNL",
    "HKD", "HUF", "ISK", "INR", "IDR", "IRR", "IQD", "ILS", "JMD", "JPY", "JOD", "KZT", "KES", "KWD", "KGS", "LBP",
    "MKD", "MYR", "MUR", "MXN", "MDL", "MNT", "MAD", "MMK", "NAD", "NPR", "TWD", "NDZ", "NIO", "NGN", "NOK", "OMR",
    "PKR", "PAB", "PEN", "PHP", "PLN", "GBP", "QAR", "RON", "RUB", "SAR", "RSD", "SGD", "ZAR", "KRW", "SSP", "VES",
    "LKR", "SEK", "CHF", "THB", "TTD", "TND", "TRY", "UGX", "UAH", "AED", "UYU", "UZS", "VND",
];

// Api providers supporting public endpoints to get crypto data
pub const API_PROVIDERS: [&str; 2] = [
    "coinmarketcap", 
    "cryptocompare"
];
