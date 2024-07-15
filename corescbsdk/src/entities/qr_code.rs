use std::fmt;
use std::fmt::Formatter;

use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::entities::validate::{validate_amount, validate_data_type_az09};

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct QRCodeRequest {
    // Type of QR Code to request for generate QR code.
    // • “PP”: QR 30
    // • “CS”: QR CS
    // • “PPCS”: QR 30 and QR CS
    #[serde(rename = "qrType")]
    qr_type: String,
    // Amount of transaction with the length up to 13 characters including "." e.g. 100, 100.00
    #[serde(rename = "amount")]
    #[validate(custom(function = "validate_amount"))]
    amount: String,
    // Invoice number as unique ID per transaction for QR CS. It must be English uppercase letters and numbers only.
    #[serde(rename = "invoice", skip_serializing_if = "Option::is_none")]
    invoice: Option<String>,
    // Merchant ID for QR CS.
    #[serde(rename = "merchantId", skip_serializing_if = "Option::is_none")]
    merchant_id: Option<String>,
    // Terminal ID for QR CS.
    #[serde(rename = "terminalId", skip_serializing_if = "Option::is_none")]
    terminal_id: Option<String>,
    // Value is in minute which indicates that the expiry time of the requested QR code will be in next X minutes
    // from the current time. If not specific the default expirytime will be 15 minutes.
    #[serde(rename = "csExtExpiryTime", skip_serializing_if = "Option::is_none")]
    cs_ext_expiry_time: Option<String>,
    // Description of QRCS Transaction
    #[serde(rename = "csNote", skip_serializing_if = "Option::is_none")]
    cs_note: Option<String>,
    // Any value which defined by user.
    #[serde(rename = "csUserDefined", skip_serializing_if = "Option::is_none")]
    cs_user_defined: Option<String>,
    // PromptPay Type for QR 30
    // Value: BILLERID
    #[serde(rename = "ppType", skip_serializing_if = "Option::is_none")]
    pp_type: Option<String>,
    // Biller ID
    //
    // Note: Partners can get on merchant profile of their application.
    // Length: 15
    #[serde(rename = "ppId", skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 15))]
    pp_id: Option<String>,
    // Reference number required for the relevant payment methods.
    // Length: up to 20
    // Data Type: [AZ09] English capital letter and number only.
    #[serde(rename = "ref1", skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 20), custom(function = "validate_data_type_az09"))]
    ref1: Option<String>,
    // Reference number required for the relevant payment methods.
    // Required if: Supporting Reference field under merchant profile of application is set to Two references.
    // Length: up to 20
    // Data Type: [AZ09] English capital letter and number only.
    #[serde(rename = "ref2", skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 20), custom(function = "validate_data_type_az09"))]
    ref2: Option<String>,
    // Reference number required for the relevant payment methods to identify endpoint for receiving payment confirmation.
    // Format: Reference 3 Prefix + (value), example: SCB1234
    //
    // Note: Partners can get the Reference 3 Prefix and set the Payment Confirmation Endpoint on merchant profile of their application.
    // Length: up to 20
    // Data Type: [AZ09] English capital letter and number only.
    #[serde(rename = "ref3", skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 20), custom(function = "validate_data_type_az09"))]
    ref3: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

pub struct QRCodeRequestBuilder {
    qrcode_request: QRCodeRequest,
}

impl QRCodeRequestBuilder {
    pub fn new(qr_type: &QRCodeType, amount: &String) -> Self {
        QRCodeRequestBuilder {
            qrcode_request: QRCodeRequest {
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
            },
        }
    }
    pub fn for_qr_cs(
        &mut self,
        invoice: &String,
        merchant_id: &String,
        terminal_id: &String,
    ) -> &mut Self {
        self.qrcode_request.invoice = Some(invoice.to_string());
        self.qrcode_request.merchant_id = Some(merchant_id.to_string());
        self.qrcode_request.terminal_id = Some(terminal_id.to_string());
        self
    }
    pub fn for_qr_tag30(
        &mut self,
        pp_type: &String,
        pp_id: &String,
        ref1: &String,
        ref3: &String,
    ) -> &mut Self {
        self.qrcode_request.pp_type = Some(pp_type.to_string());
        self.qrcode_request.pp_id = Some(pp_id.to_string());
        self.qrcode_request.ref1 = Some(ref1.to_string());
        self.qrcode_request.ref3 = Some(ref3.to_string());
        self
    }
    pub fn add_cs_ext_expiry_time(&mut self, expiry_time: &String) -> &mut Self {
        self.qrcode_request.cs_ext_expiry_time = Some(expiry_time.to_string());
        self
    }
    pub fn add_cs_note(&mut self, note: &String) -> &mut Self {
        self.qrcode_request.cs_note = Some(note.to_string());
        self
    }
    pub fn add_cs_user_defined(&mut self, user_defined: &String) -> &mut Self {
        self.qrcode_request.cs_user_defined = Some(user_defined.to_string());
        self
    }
    pub fn add_ref2(&mut self, ref2: &String) -> &mut Self {
        self.qrcode_request.ref2 = Some(ref2.to_string());
        self
    }

    pub fn build(&self) -> Result<QRCodeRequest, &'static str> {
        Ok(self.qrcode_request.clone())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QRCodeResponse {
    // QR Data
    #[serde(rename = "qrRawData")]
    pub qr_raw_data: Option<String>,
    // Base64 Image of QR
    #[serde(rename = "qrImage")]
    pub qr_image: Option<String>,
    // Conditional
    // QR CS : Expiry time of the QR code
    #[serde(rename = "csExtExpiryTime")]
    pub cs_ext_expiry_time: Option<String>,
    // Conditional
    // QR CS : Status “000” is Success
    #[serde(rename = "responseCode")]
    pub response_code: Option<String>,
    // Conditional
    // QR CS : Type of QR code
    #[serde(rename = "qrCodeType")]
    pub qr_code_type: Option<String>,
    // Conditional
    // QR CS : Unique id of QR code
    #[serde(rename = "qrCodeId")]
    pub qr_code_id: Option<String>,
    // Conditional
    // QR CS : Point of initiation method
    // - 11 = Static QR
    // - 12 = Dynamic QR
    #[serde(rename = "poi")]
    pub poi: Option<String>,
    // Conditional
    // QR CS : Amount to be paid for the transaction
    #[serde(rename = "amount")]
    pub amount: Option<String>,
    // Conditional
    // QR CS : Currency code as defined by ISO4217
    // - 764 (Baht)
    #[serde(rename = "currencyCode")]
    pub currency_code: Option<String>,
    // Conditional
    // QR CS : Name of currency
    #[serde(rename = "currencyName")]
    pub currency_name: Option<String>,
    // Conditional
    // QR CS : Description of QR transaction
    #[serde(rename = "csNote")]
    pub cs_note: Option<String>,
    // Conditional
    // QR CS : Invoice or bill number for the transaction
    #[serde(rename = "invoice")]
    pub invoice: Option<String>,
    // Conditional
    // QR CS : Merchant ID set by the company
    #[serde(rename = "merchantId")]
    pub merchant_id: Option<String>,
    // Conditional
    // QR CS : Merchant name
    #[serde(rename = "merchantName")]
    pub merchant_name: Option<String>,
    // Conditional
    // QR CS : Any value that was defined by user
    #[serde(rename = "csUserDefined")]
    pub cs_user_defined: Option<String>,
    // Conditional
    // QR CS : Terminal unique ID
    #[serde(rename = "terminalId")]
    pub terminal_id: Option<String>,
    // Conditional
    // QR CS : Name of terminal
    #[serde(rename = "terminalName")]
    pub terminal_name: Option<String>,
    // Conditional
    // QR CS : Array of supported for this QR code
    #[serde(rename = "channels")]
    pub channels: Option<Vec<Channel>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Channel {
    // Conditional
    // QR CS : Sequence number of this object in array
    #[serde(rename = "seqNo")]
    pub seq_no: Option<String>,
    // Conditional
    // QR CS : Channel code
    #[serde(rename = "channelName")]
    pub channel_name: Option<String>,
    // Conditional
    // QR CS : Channel name
    #[serde(rename = "channelCode")]
    pub channel_code: Option<String>,
}
