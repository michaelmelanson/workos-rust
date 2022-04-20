use thiserror::Error;

#[derive(Debug, Error)]
pub enum WorkOsError<E> {
    #[error("operational error")]
    Operation(#[from] E),

    #[error("unauthorized")]
    Unauthorized,
}

pub type WorkOsResult<T, E> = Result<T, WorkOsError<E>>;
