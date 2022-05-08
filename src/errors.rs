use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Error {
    pub code: i32,
    pub message: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ErrorResponse {
    IntradayError {
        #[serde(rename = "apiVersion")]
        api_version: String,
        error: Error,
    },
    MarketdataError {
        #[serde(rename = "statusCode")]
        status_code: i32,
        message: String,
    },
}

impl std::fmt::Display for ErrorResponse {
    #[cfg_attr(coverage, no_coverage)]
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ErrorResponse::IntradayError { api_version, error } => {
                write!(
                    f,
                    "FugleAPI: {{ api_version:{}, code:{}, msg:{} }}",
                    api_version, error.code, error.message,
                )
            }
            ErrorResponse::MarketdataError {
                status_code,
                message,
            } => {
                write!(f, "FugleAPI: {{ code:{}, msg:{} }}", status_code, message,)
            }
        }
    }
}

impl std::error::Error for ErrorResponse {
    #[cfg_attr(coverage, no_coverage)]
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

#[cfg_attr(coverage, no_coverage)]
#[derive(Debug)]
pub enum FugleError {
    MpscSendError,
    MpscRecvError(std::sync::mpsc::RecvError),
    // error from serde_json lib
    SerdeJson(serde_json::Error),
    // error from tungstenite lib
    #[cfg(any(feature = "websocket", feature = "async-websocket"))]
    Tungstenite(tungstenite::Error),
    // error from ureq lib
    Ureq(Box<ureq::Error>),
    // error from std io
    StdIO(std::io::Error),
    // from fugle API response code, to specific errors
    // https://developer.fugle.tw/document/intraday/introduction
    // 400
    General(ErrorResponse),
    // 401
    Unauthorized,
    // 403
    RateLimitExceeded,
    // 404
    ResourceNotFound,
    // status codes not in the list
    Unknown(ErrorResponse),
}

impl std::fmt::Display for FugleError {
    #[cfg_attr(coverage, no_coverage)]
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            FugleError::SerdeJson(ref e) => write!(f, "Serde_json Lib error: {}", e),
            #[cfg(feature = "websocket")]
            FugleError::Tungstenite(ref e) => write!(f, "Tungstenite Lib error: {}", e),
            FugleError::Ureq(ref e) => write!(f, "Ureq Lib error: {}", e),
            FugleError::StdIO(ref e) => write!(f, "std io json Deserialize error: {}", e),
            FugleError::General(ref e) => write!(f, "General purpose error: {}", e),
            FugleError::Unknown(ref e) => write!(f, "Unknown error: {}", e),
            FugleError::Unauthorized => write!(f, "Unauthorized"),
            FugleError::RateLimitExceeded => write!(f, "Rate limit or quota exceeded"),
            FugleError::ResourceNotFound => write!(f, "Resource Not Found"),
            FugleError::MpscSendError => write!(f, "MPSC Send Error"),
            FugleError::MpscRecvError(ref e) => write!(f, "MPSC Receive Error: {}", e),
        }
    }
}

impl std::error::Error for FugleError {
    #[cfg_attr(coverage, no_coverage)]
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            FugleError::SerdeJson(ref e) => Some(e),
            #[cfg(feature = "websocket")]
            FugleError::Tungstenite(ref e) => Some(e),
            FugleError::Ureq(ref e) => Some(e),
            FugleError::StdIO(ref e) => Some(e),
            FugleError::General(ref _e) => None,
            FugleError::Unknown(ref _e) => None,
            FugleError::Unauthorized => None,
            FugleError::RateLimitExceeded => None,
            FugleError::ResourceNotFound => None,
            FugleError::MpscSendError => None,
            FugleError::MpscRecvError(ref e) => Some(e),
        }
    }
}

impl From<std::sync::mpsc::RecvError> for FugleError {
    #[cfg_attr(coverage, no_coverage)]
    fn from(err: std::sync::mpsc::RecvError) -> FugleError {
        FugleError::MpscRecvError(err)
    }
}

impl From<std::io::Error> for FugleError {
    #[cfg_attr(coverage, no_coverage)]
    fn from(err: std::io::Error) -> FugleError {
        FugleError::StdIO(err)
    }
}

impl From<ureq::Error> for FugleError {
    #[cfg_attr(coverage, no_coverage)]
    fn from(err: ureq::Error) -> FugleError {
        FugleError::Ureq(Box::new(err))
    }
}

#[cfg(feature = "websocket")]
impl From<tungstenite::Error> for FugleError {
    #[cfg_attr(coverage, no_coverage)]
    fn from(err: tungstenite::Error) -> FugleError {
        FugleError::Tungstenite(err)
    }
}

impl From<serde_json::Error> for FugleError {
    #[cfg_attr(coverage, no_coverage)]
    fn from(err: serde_json::Error) -> FugleError {
        FugleError::SerdeJson(err)
    }
}

impl From<ErrorResponse> for FugleError {
    #[cfg_attr(coverage, no_coverage)]
    fn from(err: ErrorResponse) -> FugleError {
        match err {
            ErrorResponse::IntradayError {
                api_version: _,
                ref error,
            } => match error.code {
                400 => FugleError::General(err),
                401 => FugleError::Unauthorized,
                403 => FugleError::RateLimitExceeded,
                404 => FugleError::ResourceNotFound,
                _ => FugleError::Unknown(err),
            },
            ErrorResponse::MarketdataError {
                status_code,
                message: _,
            } => match status_code {
                400 => FugleError::General(err),
                401 => FugleError::Unauthorized,
                403 => FugleError::RateLimitExceeded,
                404 => FugleError::ResourceNotFound,
                _ => FugleError::Unknown(err),
            },
        }
    }
}
