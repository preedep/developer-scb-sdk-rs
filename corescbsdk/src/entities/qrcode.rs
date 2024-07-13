use std::fmt;
use std::fmt::Formatter;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct QRCodeRequest {
    // Type of QR Code to request for generate QR code.
    // • “PP”: QR 30
    // • “CS”: QR CS
    // • “PPCS”: QR 30 and QR CS
    #[serde(rename = "qrType")]
    qr_type: String,
    // Amount of transaction with the length up to 13 characters including "." e.g. 100, 100.00
    #[serde(rename = "amount")]
    amount: String,
    // Invoice number as unique ID per transaction for QR CS. It must be English uppercase letters and numbers only.
    #[serde(rename = "invoice",skip_serializing_if = "Option::is_none")]
    invoice: Option<String>,
    // Merchant ID for QR CS.
    #[serde(rename = "merchantId",skip_serializing_if = "Option::is_none")]
    merchant_id: Option<String>,
    // Terminal ID for QR CS.
    #[serde(rename = "terminalId",skip_serializing_if = "Option::is_none")]
    terminal_id: Option<String>,
    // Value is in minute which indicates that the expiry time of the requested QR code will be in next X minutes
    // from the current time. If not specific the default expirytime will be 15 minutes.
    #[serde(rename = "csExtExpiryTime",skip_serializing_if = "Option::is_none")]
    cs_ext_expiry_time: Option<String>,
    // Description of QRCS Transaction
    #[serde(rename = "csNote",skip_serializing_if = "Option::is_none")]
    cs_note: Option<String>,
    // Any value which defined by user.
    #[serde(rename = "csUserDefined",skip_serializing_if = "Option::is_none")]
    cs_user_defined: Option<String>,
    // PromptPay Type for QR 30
    // Value: BILLERID
    #[serde(rename = "ppType",skip_serializing_if = "Option::is_none")]
    pp_type: Option<String>,
    // Biller ID
    //
    // Note: Partners can get on merchant profile of their application.
    // Length: 15
    #[serde(rename = "ppId",skip_serializing_if = "Option::is_none")]
    pp_id: Option<String>,
    // Reference number required for the relevant payment methods.
    // Length: up to 20
    // Data Type: [AZ09] English capital letter and number only.
    #[serde(rename = "ref1",skip_serializing_if = "Option::is_none")]
    ref1: Option<String>,
    // Reference number required for the relevant payment methods.
    // Required if: Supporting Reference field under merchant profile of application is set to Two references.
    // Length: up to 20
    // Data Type: [AZ09] English capital letter and number only.
    #[serde(rename = "ref2",skip_serializing_if = "Option::is_none")]
    ref2: Option<String>,
    // Reference number required for the relevant payment methods to identify endpoint for receiving payment confirmation.
    // Format: Reference 3 Prefix + (value), example: SCB1234
    // 
    // Note: Partners can get the Reference 3 Prefix and set the Payment Confirmation Endpoint on merchant profile of their application.
    // Length: up to 20
    // Data Type: [AZ09] English capital letter and number only.
    #[serde(rename = "ref3",skip_serializing_if = "Option::is_none")]
    ref3: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum QRCodeType {
    // QR 30
    PP,
    // QR CS
    CS,
    // QR 30 and QR CS
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
            pp_type: None,
            pp_id: None,
            ref1: None,
            ref2: None,
            ref3: None,
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
