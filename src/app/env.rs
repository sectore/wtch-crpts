use dotenv::dotenv;
use std::env;
use std::sync::{Once, ONCE_INIT};

use super::errors;

static INIT_ENV: Once = ONCE_INIT;
pub const ENV_COINMARKETCAP_KEY: &str = "COINMARKETCAP_KEY";
pub const HEADER_COINMARKETCAP_KEY: &str = "X-CMC_PRO_API_KEY";

pub fn get_env(key: &str) -> Result<String, errors::AppError> {
    INIT_ENV.call_once(|| {
        dotenv().ok();
    });
    env::var(key.to_string()).map_err(|_| errors::AppError::Env { name: key.into() })
}
