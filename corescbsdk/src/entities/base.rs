use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize,Deserialize)]
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
    access_token: String,
    #[serde(rename = "tokenType")]
    token_type: String,
    #[serde(rename = "expiresIn")]
    expires_in: i32,
    #[serde(rename = "expiresAt")]
    expires_at: i64,
    #[serde(rename = "refreshToken")]
    refresh_token: Option<String>,
    #[serde(rename = "refreshExpiresIn")]
    refresh_expires_in: Option<i32>,
    #[serde(rename = "refreshExpiresAt")]
    refresh_expires_at: Option<i64>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct SCBResponse<T> {
    #[serde(rename = "status")]
    status: Status,
    #[serde(rename = "data")]
    data: T,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Status {
    #[serde(rename = "code")]
    code: i32,
    #[serde(rename = "description")]
    description: String,
}
