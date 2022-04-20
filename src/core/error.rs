use thiserror::Error;

#[derive(Debug, Error)]
pub enum WorkOsError<E> {
    #[error("operational error")]
    Operation(E),

    #[error("unauthorized")]
    Unauthorized,

    #[error("URL parse error")]
    UrlParseError(#[from] url::ParseError),

    #[error("request error")]
    RequestError(#[from] reqwest::Error),
}

pub type WorkOsResult<T, E> = Result<T, WorkOsError<E>>;
