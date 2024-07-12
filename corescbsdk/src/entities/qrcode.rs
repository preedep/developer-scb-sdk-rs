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

impl QRCodeRequest {
    pub fn new(qr_type: &String, amount: &String) -> QRCodeRequest {
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
    pub fn for_qr_cs(&mut self,
                     invoice: &String,
                     merchant_id: &String,
                     terminal_id: &String,
    ) -> self {
        self.invoice = Some(invoice.to_string());
        self.merchant_id = Some(merchant_id.to_string());
        self.terminal_id = Some(terminal_id.to_string());
        self
    }
    pub fn for_qr_tag30(&self) -> self {
        self
    }
    pub fn add_cs_ext_expiry_time(&mut self, expiry_time: &String) -> self {
        self.cs_ext_expiry_time = Some(expiry_time.to_string());
        self
    }
    pub fn add_cs_note(&mut self, note: &String) -> self {
        self.cs_note = Some(note.to_string());
        self
    }
    pub fn add_cs_user_defined(&mut self, user_defined: &String) -> self {
        self.cs_user_defined = Some(user_defined.to_string());
        self
    }
}

