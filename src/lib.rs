const ARKAIVE: &str = "https://arkaive.com";

#[derive(Debug)]
pub enum Error{
    ConnectionError(reqwest::Error),
    UnexpectedStatusCode(reqwest::StatusCode),
    NotAuthenticated,
    UnexpectedResponse,
    ParseError,
}

pub mod auth;
pub mod utils;
pub mod config;
