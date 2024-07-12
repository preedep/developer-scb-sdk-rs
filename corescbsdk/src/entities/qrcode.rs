use std::fmt;
use std::fmt::Formatter;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct QRCodeRequest {
    #[serde(rename = "qrType")]
    qr_type: String,
    #[serde(rename = "amount")]
    amount: String,
    #[serde(rename = "invoice")]
    invoice: Option<String>,
    #[serde(rename = "merchantId")]
    merchant_id: Option<String>,
    #[serde(rename = "terminalId")]
    terminal_id: Option<String>,
    #[serde(rename = "csExtExpiryTime")]
    cs_ext_expiry_time: Option<String>,
    #[serde(rename = "csNote")]
    cs_note: Option<String>,
    #[serde(rename = "csUserDefined")]
    cs_user_defined: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum QRCodeType {
    PP,
    CS,
    PPCS,
}
impl fmt::Display for QRCodeType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            QRCodeType::PP => write!(f, "PP"),
            QRCodeType::CS => write!(f, "CS"),
            QRCodeType::PPCS => write!(f, "PPCS"),
        }
    }
}
impl QRCodeRequest {
    pub fn new(qr_type: &QRCodeType, amount: &String) -> QRCodeRequest {
        QRCodeRequest {
            qr_type: qr_type.to_string(),
            amount: amount.to_string(),
            invoice: None,
            merchant_id: None,
            terminal_id: None,
            cs_ext_expiry_time: None,
            cs_note: None,
            cs_user_defined: None,
        }
    }
    pub fn for_qr_cs(
        &mut self,
        invoice: &String,
        merchant_id: &String,
        terminal_id: &String,
    ) -> &mut Self {
        self.invoice = Some(invoice.to_string());
        self.merchant_id = Some(merchant_id.to_string());
        self.terminal_id = Some(terminal_id.to_string());
        self
    }
    pub fn for_qr_tag30(&mut self) -> &mut Self {
        self
    }
    pub fn add_cs_ext_expiry_time(&mut self, expiry_time: &String) -> &mut Self {
        self.cs_ext_expiry_time = Some(expiry_time.to_string());
        self
    }
    pub fn add_cs_note(&mut self, note: &String) -> &mut Self {
        self.cs_note = Some(note.to_string());
        self
    }
    pub fn add_cs_user_defined(&mut self, user_defined: &String) -> &mut Self {
        self.cs_user_defined = Some(user_defined.to_string());
        self
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QRCodeResponse {
    #[serde(rename = "qrRawData")]
    pub qr_raw_data: Option<String>,
    #[serde(rename = "qrImage")]
    pub qr_image: Option<String>,
    #[serde(rename = "csExtExpiryTime")]
    pub cs_ext_expiry_time: Option<String>,
    #[serde(rename = "responseCode")]
    pub response_code: Option<String>,
    #[serde(rename = "qrCodeType")]
    pub qr_code_type: Option<String>,
    #[serde(rename = "qrCodeId")]
    pub qr_code_id: Option<String>,
    #[serde(rename = "poi")]
    pub poi: Option<String>,
    #[serde(rename = "amount")]
    pub amount: Option<String>,
    #[serde(rename = "currencyCode")]
    pub currency_code: Option<String>,
    #[serde(rename = "currencyName")]
    pub currency_name: Option<String>,
    #[serde(rename = "csNote")]
    pub cs_note: Option<String>,
    #[serde(rename = "invoice")]
    pub invoice: Option<String>,
    #[serde(rename = "merchantId")]
    pub merchant_id: Option<String>,
    #[serde(rename = "merchantName")]
    pub merchant_name: Option<String>,
    #[serde(rename = "csUserDefined")]
    pub cs_user_defined: Option<String>,
    #[serde(rename = "terminalId")]
    pub terminal_id: Option<String>,
    #[serde(rename = "terminalName")]
    pub terminal_name: Option<String>,
    #[serde(rename = "channels")]
    pub channels: Option<Vec<Channel>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Channel {
    #[serde(rename = "seqNo")]
    pub seq_no: Option<String>,
    #[serde(rename = "channelName")]
    pub channel_name: Option<String>,
    #[serde(rename = "channelCode")]
    pub channel_code: Option<String>,
}
