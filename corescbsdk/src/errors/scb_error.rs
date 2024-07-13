use std::fmt;

#[derive(Debug)]
pub enum SCBAPIError {
    HttpRequestError(reqwest::Error),
    SCBError(String),
}
impl From<reqwest::Error> for SCBAPIError {
    fn from(error: reqwest::Error) -> Self {
        SCBAPIError::HttpRequestError(error)
    }
}
impl From<String> for SCBAPIError {
    fn from(error: String) -> Self {
        SCBAPIError::SCBError(error)
    }
}

impl fmt::Display for SCBAPIError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SCBAPIError::HttpRequestError(e) => write!(f, "HTTP Request Error: {}", e),
            SCBAPIError::SCBError(e) => write!(f, "SCB Error: {}", e),
        }
    }
}
