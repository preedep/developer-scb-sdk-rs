use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SCBAccessTokenRequest {
    #[serde(rename = "applicationKey")]
    pub(crate) application_key: String,
    #[serde(rename = "applicationSecret")]
    pub(crate) application_secret: String,
    #[serde(rename = "authCode")]
    pub(crate) auth_code: Option<String>,
    #[serde(rename = "state")]
    pub(crate) state: Option<String>,
    #[serde(rename = "codeChallenge")]
    pub(crate) code_challenge: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessToken {
    #[serde(rename = "accessToken")]
    pub(crate) access_token: String,
    #[serde(rename = "tokenType")]
    pub(crate)  token_type: String,
    #[serde(rename = "expiresIn")]
    pub(crate)  expires_in: i32,
    #[serde(rename = "expiresAt")]
    pub(crate)  expires_at: i64,
    #[serde(rename = "refreshToken")]
    pub(crate)  refresh_token: Option<String>,
    #[serde(rename = "refreshExpiresIn")]
    pub(crate)  refresh_expires_in: Option<i32>,
    #[serde(rename = "refreshExpiresAt")]
    pub(crate)  refresh_expires_at: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SCBResponse<T> {
    #[serde(rename = "status")]
    pub(crate) status: Status,
    #[serde(rename = "data")]
    pub(crate) data: Option<T>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Status {
    #[serde(rename = "code")]
    pub(crate) code: i32,
    #[serde(rename = "description")]
    pub(crate) description: String,
}
