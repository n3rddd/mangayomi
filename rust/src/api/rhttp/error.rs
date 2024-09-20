use crate::api::rhttp::http::HttpResponseBody;
use std::fmt::Display;

#[derive(Clone, Debug)]
pub enum RhttpError {
    RhttpCancelError,
    RhttpTimeoutError,
    RhttpRedirectError,
    RhttpStatusCodeError(u16, Vec<(String, String)>, HttpResponseBody),
    RhttpInvalidCertificateError(String),
    RhttpConnectionError(String),
    RhttpUnknownError(String),
}

impl Display for RhttpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RhttpError::RhttpCancelError => write!(f, "RhttpCancelError"),
            RhttpError::RhttpTimeoutError => write!(f, "RhttpTimeoutError"),
            RhttpError::RhttpRedirectError => write!(f, "RhttpRedirectError"),
            RhttpError::RhttpStatusCodeError(i, _, _) => {
                write!(f, "RhttpStatusCodeError: {i}")
            }
            RhttpError::RhttpInvalidCertificateError(d) => {
                write!(f, "RhttpInvalidCertificateError: {d}")
            }
            RhttpError::RhttpConnectionError(e) => write!(f, "RhttpConnectionError: {e}"),
            RhttpError::RhttpUnknownError(e) => write!(f, "{}", e),
        }
    }
}

impl std::error::Error for RhttpError {}