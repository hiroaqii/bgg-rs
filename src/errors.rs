use thiserror::Error;

#[derive(Debug, Error)]
#[error(transparent)]
pub enum BggError {
    HttpResponse(HttpResponseError),
}

#[derive(Debug, Error)]
pub enum HttpResponseError {
    #[error("{0}")]
    Various(String),
    #[error("403 Forbidden")]
    Forbidden,
    #[error("500 Internal Server Error")]
    InternalServerError,
    #[error("503 Service Unavailable")]
    ServiceUnavailable,
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
}
