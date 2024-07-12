use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize,Deserialize)]
pub struct SCBAccessTokenRequest {
    #[serde(rename = "applicationKey")]
    application_key: String,
    #[serde(rename = "applicationSecret")]
    application_secret: String,
    #[serde(rename = "authCode")]
    auth_code: Option<String>,
    #[serde(rename = "state")]
    state: Option<String>,
    #[serde(rename = "codeChallenge")]
    code_challenge: Option<String>,
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
