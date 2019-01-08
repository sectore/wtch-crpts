#[derive(Debug, Fail)]
pub enum AppError {
    #[fail(display = "Failed to get envoirenment variable {}", name)]
    Env { name: String },
    #[fail(display = "serde error")]
    SerdeError(#[cause] serde_json::Error),
    #[fail(display = "Request to Api failed {:?}", _0)]
    ApiRequest(#[cause] reqwest::Error),
    #[fail(display = "Parsing url failed {:?}", _0)]
    ApiParseUrl(#[cause] reqwest::UrlError),
    #[fail(display = "Failed to parse value of {} from list", key)]
    ApiParseMap { key: String },
    #[fail(display = "Current coin does not exist")]
    CurrentCoinMissing(),
    #[fail(display = "Terminal IO error {:?}", _0)]
    Terminal(#[cause] std::io::Error),
}

impl From<serde_json::Error> for AppError {
    fn from(e: serde_json::Error) -> Self {
        AppError::SerdeError(e)
    }
}

impl From<reqwest::Error> for AppError {
    fn from(e: reqwest::Error) -> Self {
        AppError::ApiRequest(e)
    }
}
